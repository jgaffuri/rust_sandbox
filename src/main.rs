use anyhow::Result;
use gdal::vector::{FieldValue};
use geos::Geom;
use std::time::Instant;

use geo::{Euclidean, Distance, Point};
use geo::{MinimumRotatedRect};

pub mod ragen;

use crate::ragen::io::{load_features, save_features, LoadFeaturesOptions};
use crate::ragen::geom_utils::{geo_to_geos, geos_to_geo};


/*
TODO
elongation measure
improve geo-geos conversion: choice. test performance ?
graph model
translate some geometrical algorithms ?
schematic generalisation
*/


fn main() -> Result<()> {
    //let input_path = "/home/juju/geodata/gisco/NUTS_RG_01M_2021_3035.gpkg";
    let input_path = "/home/juju/geodata/eurogeographics/EBM/EBM_LAU_2026_2.gpkg";

    let start = Instant::now();
    println!("elapsed: {:?}", start.elapsed());

    let mut records = load_features(
        input_path,
        LoadFeaturesOptions {
            layer_name: None,
            bbox: None,
            attribute_filter: None, //Some("LEVL_CODE = 0"),
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

        let g = Clone::clone(&f.geometry); //f.geometry.buffer(5000.0, 2)?;
        let g = ragen::geom_utils::to_multi(g)?;
        f.geometry = g;

        let circularity = 4.0 * std::f64::consts::PI * f.geometry.area()? / (f.geometry.length()? * f.geometry.length()?);
        f.attributes.insert("circularity".to_string(), FieldValue::IntegerValue((circularity*100.0).round() as i32));

        //print geometry type
        //println!("Geometry type: {:?}", f.geometry.geometry_type());
        //print geometry number of points
        //println!("Geometry number of points: {:?}", f.geometry.get_num_coordinates()?);
        //print geometry first point
        //let coord_seq = f.geometry.get_geometry_n(0)?.get_exterior_ring()?.get_coord_seq()?;
        //println!("Geometry first point: {:?}", coord_seq.get_x(0).and_then(|x| coord_seq.get_y(0).map(|y| (x, y))));

        //let geo_geom: GeoGeometry<f64> = (&f.geometry).try_into()?;
        //let g = g.minimum_rotated_rectangle()?;
        //f.geometry = g;
        let g_geo = geos_to_geo(&f.geometry)?;
        let gg = g_geo.minimum_rotated_rect();
        if let Some(gg) = gg {
            let gg = geo_to_geos(&geo_types::Geometry::Polygon(gg));
            let mmbra = gg?.area()?;

            let rectangularity = f.geometry.area()? / mmbra;
            f.attributes.insert("rectangularity".to_string(), FieldValue::IntegerValue((rectangularity*100.0).round() as i32));

            //f.geometry = gg?;
            //println!("{:?}", gg);
        }
        let elongation = elongation_measure(&g_geo)?;
        f.attributes.insert("elongation".to_string(), FieldValue::RealValue(elongation));
    }

    println!("Modified {} features", records.len());
    println!("elapsed: {:?}", start.elapsed());

    let output_path = "/home/juju/Bureau/rust_out.gpkg";
    save_features(&records, output_path, Some(3035), None)?;

    println!("Wrote translated features to {}", output_path);
    println!("elapsed: {:?}", start.elapsed());

    Ok(())
}

/*
use geo::EuclideanLength;

let mut segments = linestring.lines();

let first_len = segments.next().map(|l| l.euclidean_length());
let second_len = segments.next().map(|l| l.euclidean_length());

println!("{:?}, {:?}", first_len, second_len);
*/


fn elongation_measure(geometry: &geo_types::Geometry<f64>) -> Result<f64> {
    let min_rot_rect = geometry.minimum_rotated_rect();
    if let Some(min_rot_rect) = min_rot_rect {
        let coords = min_rot_rect.exterior().coords().collect::<Vec<_>>();
        if coords.len() >= 4 {
            let p0: Point = Point::new(coords[0].x, coords[0].y);
            let p1: Point = Point::new(coords[1].x, coords[1].y);
            let p2: Point = Point::new(coords[2].x, coords[2].y);
            let width = Euclidean.distance(p0, p1);
            let height = Euclidean.distance(p1, p2);
            if width > 0.0 && height > 0.0 {
                return Ok(width.min(height) / width.max(height));
            }
        }
    }
    Ok(0.0)
}
