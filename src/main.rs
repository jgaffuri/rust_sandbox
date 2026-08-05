use anyhow::Result;
//use gdal::vector::{FieldValue};
use geos::Geom;
use std::time::Instant;
use wkt::TryFromWkt;

use geo::{Geometry as GeoGeometry, MinimumRotatedRect};
//use geo::algorithm::wkt::ToWkt;

pub mod ragen;

use crate::ragen::io::{load_features, save_features, LoadFeaturesOptions};


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

        //print geometry type
        println!("Geometry type: {:?}", f.geometry.geometry_type());
        //print geometry number of points
        println!("Geometry number of points: {:?}", f.geometry.get_num_coordinates()?);
        //print geometry first point
        let coord_seq = f.geometry.get_geometry_n(0)?.get_exterior_ring()?.get_coord_seq()?;
        println!("Geometry first point: {:?}", coord_seq.get_x(0).and_then(|x| coord_seq.get_y(0).map(|y| (x, y))));

        //let geo_geom: GeoGeometry<f64> = (&f.geometry).try_into()?;
        //let g = g.minimum_rotated_rectangle()?;
        //f.geometry = g;
        let gg = geos_to_geo(&f.geometry)?;
        let gg = gg.minimum_rotated_rect();
    }

    println!("Modified {} features", records.len());
    println!("elapsed: {:?}", start.elapsed());

    let output_path = "/home/juju/Bureau/rust_out.gpkg";
    save_features(&records, output_path, Some(3035), None)?;

    println!("Wrote translated features to {}", output_path);
    println!("elapsed: {:?}", start.elapsed());

    Ok(())
}



fn geos_to_geo(geom: &geos::Geometry) -> Result<geo_types::Geometry<f64>> {
    let wkt_str = geom.to_wkt()?;
    let geo_geom = geo_types::Geometry::<f64>::try_from_wkt_str(&wkt_str).map_err(|e| anyhow::anyhow!("{}", e))?;
    Ok(geo_geom)
}

/*
fn geo_to_geos(geom: &geo_types::Geometry<f64>) -> Result<geos::Geometry, Box<dyn std::error::Error>> {
    //use geo::algorithm::to_wkt::ToWkt; // for geo_types -> WKT string
    let wkt_str = geom.to_wkt().to_string();
    let geos_geom = geos::Geometry::new_from_wkt(&wkt_str)?;
    Ok(geos_geom)
}
*/