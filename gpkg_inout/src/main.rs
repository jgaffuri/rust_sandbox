use anyhow::{Result, anyhow};
use gdal::vector::{Feature as GDALFeature, FieldValue, LayerAccess, LayerOptions};
use gdal::{Dataset, DriverManager};
use gdal::spatial_ref::SpatialRef;
use geos::Geom;
use std::collections::HashMap;

/*
TODO
make schema from feature collection
CRS from EPSG
generic save GPKG function from feature collection
extract load/save functions to separate module
*/

pub struct Feature {
    pub geometry: geos::Geometry,
    pub fields: HashMap<String, Option<FieldValue>>,
    pub fields2: HashMap<String, FieldValue>,
}

#[derive(Default)]
pub struct LoadFeaturesOptions<'a> {
    pub layer_name: Option<&'a str>,
    pub bbox: Option<&'a [f64; 4]>,
    pub attribute_filter: Option<&'a str>,
}

pub fn load_features(path: &str, options: LoadFeaturesOptions<'_>) -> Result<Vec<Feature>> {
    let dataset = Dataset::open(path)?;

    let mut layer = match options.layer_name {
        Some(name) => dataset.layer_by_name(name)?,
        None => dataset.layer(0)?,
    };

    if let Some(bbox) = options.bbox {
        layer.set_spatial_filter_rect(bbox[0], bbox[1], bbox[2], bbox[3]);
    }

    if let Some(filter) = options.attribute_filter {
        layer.set_attribute_filter(filter)?;
    }

    layer
        .features()
        .map(|feature| {
            let g = feature
                .geometry()
                .ok_or_else(|| anyhow!("feature has no geometry"))?;

            let g: Vec<u8> = g.wkb()?;
            let g = geos::Geometry::new_from_wkb(&g)?;

            let mut fields: HashMap<String, FieldValue> = HashMap::new();
            for fff in feature.fields() {
                let key = fff.0;
                let value = fff.1.unwrap();
                fields.insert(key, value);
                //println!("{}    {:?}", key, value)
            }

            Ok(Feature {
                geometry: g,
                fields: feature.fields().collect(),
                fields2: fields,
            })
        })
        .collect()
}


pub fn save_features(fs: &Vec<Feature>, path: &str, md: &GPKGMetadata, epsg: u32) -> Result<()> {

    let output_layer_name = "lay";
    let srs_projected = SpatialRef::from_epsg(epsg)?;

    //
    let driver = DriverManager::get_driver_by_name("GPKG")?;
    let mut out_dataset = driver.create_vector_only(path)?;

    let out_layer = out_dataset.create_layer(LayerOptions {
        name: output_layer_name,
        srs: Some(&srs_projected),
        ty: md.geom_field_type,
        ..Default::default()
    })?;

    // Recreate the attribute schema on the output layer.
    for (name, field_type) in &md.field_defs {
        out_layer.create_defn_fields(&[(name.as_str(), *field_type)])?;
    }

    let field_indices: HashMap<String, usize> = out_layer
        .defn()
        .fields()
        .enumerate()
        .map(|(i, field)| (field.name(), i))
        .collect();

    for record in fs {
        let mut feature = GDALFeature::new(out_layer.defn())?;

        //let ggg = record.geometry.to_gdal()?;
        //let ogr_geom = gdal::vector::Geometry::try_from(record.geometry)
        //    .expect("could not convert geo_types geometry back to OGR");

        let wkb = record.geometry.to_wkb()?;
        let ggg = gdal::vector::Geometry::from_wkb(&wkb)?;
        feature.set_geometry(ggg)?;

        for (name, value) in &record.fields {
            if let (Some(value), Some(&index)) = (value, field_indices.get(name)) {
                feature.set_field(index, value)?;
            }
        }

        /*
        for (name, value) in &record.fields {
            if let Some(value) = value {
                if out_layer.defn().field_index(name).is_ok() {
                    feature.set_field(name, value)?;
                }
            }
        }

        for (name, value) in &record.fields {
            if let Some(v) = value {
                feature.set_field(name, v)?;
            }
        }*/

        feature.create(&out_layer)?;
    }

    Ok(())
}


pub struct GPKGMetadata {
    //pub srs: Option<gdal::spatial_ref::SpatialRef>,
    pub geom_field_type: u32,
    pub field_defs: Vec<(String, gdal::vector::OGRFieldType::Type)>,
}

fn get_metadata(path: &str) -> GPKGMetadata {
    let dataset: Dataset = Dataset::open(path).unwrap();
    let layer: gdal::vector::Layer<'_> = dataset.layer(0).unwrap();
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
    GPKGMetadata {
        geom_field_type: geom_field_type,
        field_defs: field_defs,
    }
}

fn main() -> Result<()> {
    let input_path = "/home/juju/geodata/gisco/NUTS_RG_01M_2021_3035.gpkg";

    let mut records = load_features(
        input_path,
        LoadFeaturesOptions {
            layer_name: None,
            bbox: None,
            attribute_filter: Some("LEVL_CODE = 0"),
        },
    )?;

    println!("Loaded {} features from {}", records.len(), input_path);

    for f in &mut records {
        if let Some(FieldValue::StringValue(s)) = f.fields2.get("NUTS_ID") {
            println!("NUTS_ID: {}", s);
        } else {
            println!("NUTS_ID: <missing>");
        }
        //println!("{:?}", f.geometry);
        //print_type_of(&f.geometry);
        //f.geometry = geo_types::Geometry::MultiPolygon(f.geometry.buffer(50.0));
        f.geometry = f.geometry.buffer(5000.0, 2)?;
    }

    println!("Modified {} features", records.len());

    let output_path = "/home/juju/Bureau/rust_out.gpkg";
    let md = get_metadata(input_path);
    save_features(&records, output_path, &md, 3035)?;

    println!("Wrote translated features to {}", output_path);

    Ok(())
}

