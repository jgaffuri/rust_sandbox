use anyhow::Result;
//use gdal::errors::Result;
use gdal::vector::{Feature, FieldValue, Geometry, LayerAccess, LayerOptions};
use gdal::{Dataset, DriverManager};
use geo::{Buffer, Translate};
use std::any::Any;
use std::convert::TryFrom;
use std::ops::Deref;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

#[derive(Debug)]
struct FeatureRecord {
    geometry: geo_types::Geometry<f64>,
    //TODO make it a hashmap instead
    fields: Vec<(String, Option<FieldValue>)>,
}

pub struct LoadFeaturesOptions<'a> {
    pub layer_name: Option<&'a str>,
    pub bbox: Option<(f64, f64, f64, f64)>,
    pub attribute_filter: Option<&'a str>,
}

impl Default for LoadFeaturesOptions<'_> {
    fn default() -> Self {
        Self {
            layer_name: None,
            bbox: None,
            attribute_filter: None,
        }
    }
}

pub fn load_features(path: &str, options: LoadFeaturesOptions<'_>) -> Result<Vec<FeatureRecord>> {
    let dataset = Dataset::open(path)?;

    let mut layer = match options.layer_name {
        Some(name) => dataset.layer_by_name(name)?,
        None => dataset.layer(0)?,
    };

    // Apply spatial filter
    if let Some((minx, miny, maxx, maxy)) = options.bbox {
        layer.set_spatial_filter_rect(minx, miny, maxx, maxy);
    }

    // Apply attribute (SQL WHERE clause) filter
    if let Some(filter) = options.attribute_filter {
        layer.set_attribute_filter(filter)?;
    }

    let mut out = Vec::new();

    for feature in layer.features() {
        let ogr_geom = feature.geometry().expect("feature has no geometry").clone();

        let geo_geom = geo_types::Geometry::<f64>::try_from(ogr_geom)
            .expect("could not convert OGR geometry to geo_types");

        let fields = feature.fields().collect();

        out.push(FeatureRecord {
            fields,
            geometry: geo_geom,
        });
    }

    Ok(out)
}

fn main() -> Result<()> {
    let input_path = "/home/juju/geodata/gisco/NUTS_RG_01M_2021_3035.gpkg";

    let records = load_features(
        input_path,
        LoadFeaturesOptions {
            layer_name: None,
            bbox: None,
            attribute_filter: Some("LEVL_CODE = 0"),
        },
    )?;

    println!("Loaded {} features from {}", records.len(), input_path);

    println!("Done");

    for mut f in records {
        println!("{:?}", f.fields);
        //println!("{:?}", f.geometry);
        print_type_of(&f.geometry);
        f.geometry = geo_types::Geometry::MultiPolygon(f.geometry.buffer(50.0));
    }

    let output_path = "/home/juju/Bureau/rust_out.gpkg";
    let output_layer_name = "lay";

    //
    let dataset = Dataset::open(input_path)?;
    let layer = dataset.layer(0)?;
    let srs = layer.spatial_ref();
    let geom_field_type = layer
        .defn()
        .geom_fields()
        .next()
        .map(|f| f.field_type())
        .unwrap_or(gdal_sys::OGRwkbGeometryType::wkbUnknown);
    let field_defs: Vec<(String, gdal::vector::OGRFieldType::Type)> = layer
        .defn()
        .fields()
        .map(|f| (f.name(), f.field_type()))
        .collect();

    let driver = DriverManager::get_driver_by_name("GPKG")?;
    let mut out_dataset = driver.create_vector_only(output_path)?;

    let mut out_layer = out_dataset.create_layer(LayerOptions {
        name: output_layer_name,
        srs: srs.as_ref(),
        ty: geom_field_type,
        ..Default::default()
    })?;

    // Recreate the attribute schema on the output layer.
    for (name, field_type) in &field_defs {
        out_layer.create_defn_fields(&[(name.as_str(), *field_type)])?;
    }

    for record in records {
        let mut feature = Feature::new(out_layer.defn())?;

        let ogr_geom = Geometry::try_from(record.geometry)
            .expect("could not convert geo_types geometry back to OGR");
        feature.set_geometry(ogr_geom)?;

        for (name, value) in &record.fields {
            if let Some(v) = value {
                feature.set_field(name, v)?;
            }
        }

        feature.create(&out_layer)?;
    }

    println!("Wrote translated features to {}", output_path);

    Ok(())
}
