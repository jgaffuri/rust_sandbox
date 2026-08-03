use anyhow::{Result};
//use gdal::vector::{FieldValue};
use geos::Geom;
use std::time::Instant;

pub mod ragen;

use crate::ragen::iogpkg::{load_features, save_features, LoadFeaturesOptions};


/*
TODO
circularity and squarness ranking
translate some geometrical algorithms
schematic generalisation
*/


fn main() -> Result<()> {
    let input_path = "/home/juju/geodata/gisco/NUTS_RG_01M_2021_3035.gpkg";

    let start = Instant::now();
    println!("elapsed: {:?}", start.elapsed());

    let mut records = load_features(
        input_path,
        LoadFeaturesOptions {
            layer_name: None,
            bbox: None,
            attribute_filter: Some("LEVL_CODE = 0"),
        },
    )?;

    println!("Loaded {} features from {}", records.len(), input_path);
    println!("elapsed: {:?}", start.elapsed());

    for f in &mut records {
        /*if let Some(FieldValue::StringValue(s)) = f.fields.get("NUTS_ID") {
            println!("NUTS_ID: {}", s);
        } else {
            println!("NUTS_ID: <missing>");
        }*/
        //println!("{:?}", f.geometry);
        //print_type_of(&f.geometry);
        //f.geometry = geo_types::Geometry::MultiPolygon(f.geometry.buffer(50.0));

        let g = f.geometry.buffer(5000.0, 2)?;
        let g = ragen::geom_utils::to_multi(g)?;
        f.geometry = g;
    }

    println!("Modified {} features", records.len());
    println!("elapsed: {:?}", start.elapsed());

    let output_path = "/home/juju/Bureau/rust_out.gpkg";
    save_features(&records, output_path, Some(3035), None)?;

    println!("Wrote translated features to {}", output_path);
    println!("elapsed: {:?}", start.elapsed());

    Ok(())
}
