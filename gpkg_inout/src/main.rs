use gdal::errors::Result;
use gdal::vector::{Feature, FieldValue, Geometry, LayerAccess, LayerOptions};
use gdal::{Dataset, DriverManager};
use geo::{Buffer, Translate};
use std::any::Any;
use std::convert::TryFrom;
use std::ops::Deref;

/// One in-memory copy of a feature: its attribute fields plus its geometry
/// converted to `geo_types`, which is what lets us use `geo`'s algorithms.
#[derive(Debug)]
struct FeatureRecord {
    fields: Vec<(String, Option<FieldValue>)>,
    geometry: geo_types::Geometry<f64>,
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}



fn load_features(path: &str) -> Result<Vec<FeatureRecord>> {
    let dataset = Dataset::open(path).expect(&format!("failed to load dataset from {}", path));
    let mut layer = dataset.layer(0)?;

    let mut out: Vec<FeatureRecord> = Vec::new();

    for feature in layer.features() {
        let ogr_geom = feature.geometry().expect("feature has no geometry").clone();
        let geo_geom = geo_types::Geometry::<f64>::try_from(ogr_geom)
            .expect("could not convert OGR geometry to geo_types");

        let fields: Vec<(String, Option<FieldValue>)> = feature.fields().collect();

        out.push(FeatureRecord {
            fields,
            geometry: geo_geom,
        });
    }

    Ok(out)
}

fn main() -> Result<()> {
    let input_path = "/home/juju/geodata/gisco/NUTS_RG_01M_2021_3035.gpkg";

    let records = load_features(input_path)?;

    println!("Loaded {} features from {}", records.len(), input_path);

    println!("Done");

    for mut f in records {
        println!("{:?}", f.fields.get(0).ok_or("no").unwrap().1.as_ref().ok_or("no").unwrap());
        //println!("{:?}", f.geometry);
        print_type_of(&f.geometry);
        f.geometry = geo_types::Geometry::MultiPolygon(f.geometry.buffer(50.0));
    }

    Ok(())
}
