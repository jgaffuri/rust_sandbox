use gdal::errors::Result;
use gdal::vector::{Feature, FieldValue, Geometry, LayerAccess, LayerOptions};
use gdal::{Dataset, DriverManager};
use geo::Translate;
use std::convert::TryFrom;

/// One in-memory copy of a feature: its attribute fields plus its geometry
/// converted to `geo_types`, which is what lets us use `geo`'s algorithms.
struct FeatureRecord {
    fields: Vec<(String, Option<FieldValue>)>,
    geometry: geo_types::Geometry<f64>,
}

fn main() -> Result<()> {
    let input_path = "/home/juju/geodata/gisco/NUTS_RG_01M_2021_3035.gpkg";
    //let output_path = "output.gpkg";
    //let output_layer_name = "translated_layer";

    let dataset = Dataset::open(input_path)?;
    let mut layer = dataset.layer(0)?;

    // Field names/types, captured before we mutably iterate features.
    let field_defs: Vec<(String, gdal::vector::OGRFieldType::Type)> = layer
        .defn()
        .fields()
        .map(|f| (f.name(), f.field_type()))
        .collect();

    let geom_field_type = layer
        .defn()
        .geom_fields()
        .next()
        .map(|f| f.field_type())
        .unwrap_or(gdal_sys::OGRwkbGeometryType::wkbUnknown);

    //let srs = layer.spatial_ref();
    //println!("{}", srs);

    let mut records: Vec<FeatureRecord> = Vec::new();

    for feature in layer.features() {
        let ogr_geom = feature.geometry().expect("feature has no geometry").clone();
        let geo_geom = geo_types::Geometry::<f64>::try_from(ogr_geom)
            .expect("could not convert OGR geometry to geo_types");

        let mut fields = Vec::with_capacity(field_defs.len());
        for (name, _) in &field_defs {
            fields.push((name.clone(), feature.field(name)?));
        }

        records.push(FeatureRecord {
            fields,
            geometry: geo_geom,
        });
    }

    println!("Loaded {} features from {}", records.len(), input_path);

    println!("Done");

    Ok(())
}
