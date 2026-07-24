//use anyhow::Result;
use anyhow::{Result, anyhow};
//use gdal::errors::Result;
use gdal::vector::{Feature, FieldValue, Geometry, LayerAccess, LayerOptions, ToGdal};
use gdal::{Dataset, DriverManager};
use geo::{Buffer, Rect, Translate};
use std::any::Any;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::ops::Deref;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

/*
pub struct FeatureCollection {
    pub fields: Vec<FieldDef>,
    pub records: Vec<FeatureRecord>,
}*/

pub struct FeatureRecord {
    pub geometry: geo_types::Geometry<f64>,
    pub fields: HashMap<String, Option<FieldValue>>,
}

#[derive(Default)]
pub struct LoadFeaturesOptions<'a> {
    pub layer_name: Option<&'a str>,
    pub bbox: Option<Rect<f64>>,
    pub attribute_filter: Option<&'a str>,
}

pub fn load_features(path: &str, options: LoadFeaturesOptions<'_>) -> Result<Vec<FeatureRecord>> {
    let dataset = Dataset::open(path)?;

    let mut layer = match options.layer_name {
        Some(name) => dataset.layer_by_name(name)?,
        None => dataset.layer(0)?,
    };

    if let Some(rect) = options.bbox {
        layer.set_spatial_filter_rect(rect.min().x, rect.min().y, rect.max().x, rect.max().y);
    }

    if let Some(filter) = options.attribute_filter {
        layer.set_attribute_filter(filter)?;
    }

    layer
        .features()
        .map(|feature| {
            let ogr_geom = feature
                .geometry()
                .ok_or_else(|| anyhow!("feature has no geometry"))?;

            Ok(FeatureRecord {
                geometry: geo_types::Geometry::try_from(ogr_geom)?,
                fields: feature.fields().collect(),
            })
        })
        .collect()
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
        println!(
            "{:?}",
            f.fields
                .get("NUTS_ID")
                .and_then(|value| value.as_ref())
                .unwrap()
        );
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

        let ggg = record.geometry.to_gdal()?;
        //let ogr_geom = gdal::vector::Geometry::try_from(record.geometry)
        //    .expect("could not convert geo_types geometry back to OGR");
        feature.set_geometry(ggg)?;

        for (name, value) in &record.fields {
            if let Some(value) = value {
                if out_layer.defn().field_index(name).is_ok() {
                    feature.set_field(name, value)?;
                }
            }
        }

        /*
        for (name, value) in &record.fields {
            if let Some(v) = value {
                feature.set_field(name, v)?;
            }
        }*/

        feature.create(&out_layer)?;
    }

    println!("Wrote translated features to {}", output_path);

    Ok(())
}
