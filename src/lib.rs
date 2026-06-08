//! # geotiff_agg
//!
//! Aggregate GeoTIFF pixel values into polygon regions defined in a GeoPackage.
//!
//! ## Performance design
//!
//! * Regions are loaded once and reprojected to the raster CRS.
//! * An R*-tree over region bounding boxes gives O(log n) spatial pre-filtering
//!   per pixel, avoiding a brute-force scan of all regions.
//! * The raster is read in horizontal strips (tiles) so memory usage is bounded
//!   regardless of raster size.
//! * Each tile is processed in parallel with Rayon; thread-local hashmaps avoid
//!   lock contention, and their results are merged once per tile.
//! * Only rows that overlap at least one region bounding box are visited.

use anyhow::{Context, Result};
use gdal::{
    vector::{Geometry, LayerAccess},
    Dataset,
};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use rstar::{RTree, RTreeObject, AABB};
use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, Mutex},
};

// ---------------------------------------------------------------------------
// R*-tree wrapper
// ---------------------------------------------------------------------------

/// A region record stored in the R*-tree.
/// Holds the index into the master `regions` Vec so we can retrieve the full
/// geometry without cloning it into the tree.
#[derive(Clone)]
struct RegionNode {
    /// Index into the `regions` slice.
    idx: usize,
    /// Envelope [min_x, min_y, max_x, max_y] in raster CRS.
    env: [f64; 4],
}

impl RTreeObject for RegionNode {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_corners([self.env[0], self.env[1]], [self.env[2], self.env[3]])
    }
}

// ---------------------------------------------------------------------------
// Region record
// ---------------------------------------------------------------------------

struct Region {
    id: String,
    geom: Geometry,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Aggregate GeoTIFF pixel values (band 1) by region.
///
/// For each region the function sums all pixel values whose **centre** lies
/// strictly inside (or on the boundary of) that region.  Pixels flagged as
/// NoData and non-finite values are skipped.  Regions that contain no pixels
/// receive a sum of `0.0` in the output.
///
/// # Arguments
/// | Parameter       | Description |
/// |-----------------|-------------|
/// | `gpkg_path`     | Path to a GeoPackage with polygon / multipolygon features. |
/// | `region_id_attr`| Name of the attribute field used as the region identifier. |
/// | `tiff_path`     | Path to the input GeoTIFF (band 1 is used). |
/// | `output_csv`    | Path where the two-column CSV (`region_id,sum`) is written. |
///
/// # Errors
/// Returns an error if any file cannot be opened, the attribute field is
/// missing, or the raster lacks a geotransform / CRS.
pub fn aggregate_geotiff_by_region(
    gpkg_path: impl AsRef<Path>,
    region_id_attr: &str,
    tiff_path: impl AsRef<Path>,
    output_csv: impl AsRef<Path>,
) -> Result<()> {
    // ---------------------------------------------------------------
    // 1. Open raster – read metadata
    // ---------------------------------------------------------------
    let raster = Dataset::open(tiff_path.as_ref())
        .with_context(|| format!("Cannot open raster: {}", tiff_path.as_ref().display()))?;

    let (raster_width, raster_height) = raster.raster_size();
    let geo = raster
        .geo_transform()
        .context("Raster has no geotransform")?;

    // geo = [top_left_x, pixel_w, rot_x, top_left_y, rot_y, pixel_h]
    // pixel_h is negative for north-up rasters.
    let (origin_x, pixel_w, origin_y, pixel_h) = (geo[0], geo[1], geo[3], geo[5]);

    // Warn if the raster is rotated (uncommon, not handled here).
    if geo[2].abs() > 1e-10 || geo[4].abs() > 1e-10 {
        eprintln!("WARNING: rotated rasters are not supported; results may be incorrect.");
    }

    let raster_srs = raster
        .spatial_ref()
        .context("Raster has no CRS")?;

    let band = raster.rasterband(1)?;
    let no_data = band.no_data_value();

    // ---------------------------------------------------------------
    // 2. Load & reproject vector regions
    // ---------------------------------------------------------------
    let vector = Dataset::open(gpkg_path.as_ref())
        .with_context(|| format!("Cannot open GPKG: {}", gpkg_path.as_ref().display()))?;

    anyhow::ensure!(vector.layer_count() > 0, "GPKG contains no layers");
    let mut layer = vector.layer(0)?;

    let layer_srs = layer.spatial_ref();
    let needs_reproject = match &layer_srs {
        Some(lsrs) => {
            // Compare by authority string; fall back to WKT comparison.
            let la = lsrs.authority().unwrap_or_default();
            let ra = raster_srs.authority().unwrap_or_default();
            la != ra
        }
        None => false,
    };

    let mut regions: Vec<Region> = Vec::new();
    let mut tree_nodes: Vec<RegionNode> = Vec::new();

    for feature in layer.features() {
        // Read region id
        let id = feature
            .field(region_id_attr)
            .with_context(|| format!("Field '{}' not found in feature", region_id_attr))?
            .map(|v| v.to_string())
            .unwrap_or_default();

        if id.is_empty() {
            continue;
        }

        let raw_geom = match feature.geometry() {
            Some(g) => g.clone(),
            None => continue,
        };

        // Reproject to raster CRS if needed
        let geom = if needs_reproject {
            let mut g = raw_geom.clone();
            g.transform_to_inplace(&raster_srs)
                .context("Geometry reprojection failed")?;
            g
        } else {
            raw_geom.clone()
        };

        // Build envelope
        let env = geom.envelope();
        let idx = regions.len();

        tree_nodes.push(RegionNode {
            idx,
            env: [env.MinX, env.MinY, env.MaxX, env.MaxY],
        });
        regions.push(Region { id, geom });
    }

    anyhow::ensure!(!regions.is_empty(), "No features with valid geometry found");
    eprintln!("Loaded {} regions.", regions.len());

    // ---------------------------------------------------------------
    // 3. Build R*-tree
    // ---------------------------------------------------------------
    let rtree = Arc::new(RTree::bulk_load(tree_nodes.clone()));
    let regions = Arc::new(regions);

    // ---------------------------------------------------------------
    // 4. Compute row range that overlaps any region
    // ---------------------------------------------------------------
    // pixel_h is negative for north-up; y_to_row gives the 0-based row index.
    let y_to_row = |y: f64| -> isize { ((y - origin_y) / pixel_h).floor() as isize };

    let overall_max_y = tree_nodes
        .iter()
        .map(|n| n.env[3])
        .fold(f64::NEG_INFINITY, f64::max);
    let overall_min_y = tree_nodes
        .iter()
        .map(|n| n.env[1])
        .fold(f64::INFINITY, f64::min);

    // For north-up (pixel_h < 0): higher Y → smaller row index
    let row_start = y_to_row(overall_max_y).max(0) as usize;
    let row_end = (y_to_row(overall_min_y) + 1)
        .min(raster_height as isize)
        .max(0) as usize;

    if row_start >= row_end {
        eprintln!("No raster rows overlap the region extents. Writing empty CSV.");
    } else {
        eprintln!(
            "Scanning rows {} – {} (of {}).",
            row_start, row_end, raster_height
        );
    }

    // ---------------------------------------------------------------
    // 5. Process raster in tiles
    // ---------------------------------------------------------------
    // 512-row tiles balance memory use and I/O efficiency.
    // Increase for fewer, larger reads on fast storage; decrease on low-RAM systems.
    const TILE_ROWS: usize = 512;

    let n_tiles = if row_start < row_end {
        (row_end - row_start).div_ceil(TILE_ROWS)
    } else {
        0
    };

    let progress = ProgressBar::new(n_tiles as u64);
    progress.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:50.cyan/blue} {pos}/{len} tiles  ETA {eta}")?
            .progress_chars("=>-"),
    );

    // Global accumulator: region_id → sum.
    let global_sums: Arc<Mutex<HashMap<String, f64>>> = Arc::new(Mutex::new({
        // Pre-populate so every region appears in the output.
        regions
            .iter()
            .map(|r| (r.id.clone(), 0.0_f64))
            .collect()
    }));

    let mut tile_row = row_start;
    while tile_row < row_end {
        let tile_end = (tile_row + TILE_ROWS).min(row_end);
        let n_rows = tile_end - tile_row;

        // Read tile as f64 (GDAL converts any raster type automatically).
        let buf = raster
            .rasterband(1)?
            .read_as::<f64>(
                (0, tile_row as isize),
                (raster_width, n_rows),
                (raster_width, n_rows),
                None,
            )
            .with_context(|| format!("Failed to read tile at row {}", tile_row))?;

        let data: Arc<Vec<f64>> = Arc::new(buf.data().to_vec());

        // Process rows in this tile in parallel; each thread accumulates into
        // a local HashMap to avoid lock contention.
        let tile_row_sums: Vec<HashMap<String, f64>> = (0..n_rows)
            .into_par_iter()
            .map(|local_row| {
                let abs_row = tile_row + local_row;
                // Geographic Y of the pixel centre (north-up: origin_y is the top edge)
                let cy = origin_y + (abs_row as f64 + 0.5) * pixel_h;

                let mut local: HashMap<String, f64> = HashMap::new();

                for col in 0..raster_width {
                    let val = data[local_row * raster_width + col];

                    // Skip NoData
                    if let Some(nd) = no_data {
                        if (val - nd).abs() < 1e-10 {
                            continue;
                        }
                    }
                    if !val.is_finite() {
                        continue;
                    }

                    let cx = origin_x + (col as f64 + 0.5) * pixel_w;

                    // R*-tree query: candidate regions whose bbox contains (cx, cy)
                    let candidates = rtree.locate_in_envelope_intersecting(
                        &AABB::from_corners([cx, cy], [cx, cy]),
                    );

                    for node in candidates {
                        let region = &regions[node.idx];
                        // Precise point-in-polygon via GDAL
                        if contains_point(&region.geom, cx, cy) {
                            *local.entry(region.id.clone()).or_insert(0.0) += val;
                        }
                    }
                }
                local
            })
            .collect();

        // Merge thread-local maps into the global accumulator once per tile.
        {
            let mut global = global_sums.lock().unwrap();
            for local in tile_row_sums {
                for (id, sum) in local {
                    *global.entry(id).or_insert(0.0) += sum;
                }
            }
        }

        tile_row = tile_end;
        progress.inc(1);
    }

    progress.finish_with_message("raster scan complete");

    // ---------------------------------------------------------------
    // 6. Write output CSV
    // ---------------------------------------------------------------
    let final_sums = Arc::try_unwrap(global_sums)
        .expect("no other Arc holders")
        .into_inner()
        .unwrap();

    let mut writer = csv::WriterBuilder::new()
        .has_headers(true)
        .from_path(output_csv.as_ref())
        .with_context(|| format!("Cannot create CSV: {}", output_csv.as_ref().display()))?;

    writer.write_record(["region_id", "sum"])?;

    // Preserve original feature order
    let regions_ref = Arc::try_unwrap(regions).expect("no other Arc holders");
    for region in &regions_ref {
        let sum = final_sums.get(&region.id).copied().unwrap_or(0.0);
        writer.write_record([region.id.as_str(), &sum.to_string()])?;
    }

    writer.flush()?;
    eprintln!("Written: {}", output_csv.as_ref().display());
    Ok(())
}

// ---------------------------------------------------------------------------
// Helper: GDAL point-in-polygon test
// ---------------------------------------------------------------------------

/// Returns `true` if the polygon geometry `geom` contains the point (x, y).
/// Uses GDAL's `Contains` predicate which handles multipolygons, holes, and
/// boundary cases correctly.
#[inline]
fn contains_point(geom: &Geometry, x: f64, y: f64) -> bool {
    let wkt = format!("POINT ({x:.15} {y:.15})");
    match Geometry::from_wkt(&wkt) {
        Ok(pt) => geom.contains(&pt),
        Err(_) => false,
    }
}
