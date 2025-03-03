use gdal::spatial_ref::SpatialRef;
use gdal::{Dataset,Metadata,DriverManager};
use gdal::vector::{Feature, FeatureIterator, FieldDefn, FieldValue, Geometry, Layer, LayerAccess, LayerOptions, OGRFieldType, OGRwkbGeometryType};
use gdal::vector::geometry_type_to_name;
use gdal::vector::OGRwkbGeometryType::wkbPoint;
use geo::{line_string, point, polygon, Point};
use gdal::errors::GdalError;

fn main() {

    //read_gpkg("/home/juju/geodata/gisco/CNTR_RG_03M_2024_3035.gpkg", false, 0.0, 0.0, 10.0, 10.0)

    //write_gpkg("/home/juju/Bureau/rust_test.gpkg", "my_layer")

    validate_grid()

}



fn validate_grid() {
    println!("Validation");

    let cells = load_gpkg_layer(
        "/home/juju/geodata/census/2021/ESTAT_Census_2021_V2.gpkg",
        "census2021",
        3921310.0, 2233307.0,
        4006894.9, 2291515.9);
    println!("Cells {:?}", cells.len());

}



fn load_gpkg_layer(gpkg_path: &str, layer_name: &str, min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> Vec<Geometry> {

    let dataset: Dataset = Dataset::open(gpkg_path).unwrap();
    println!("Dataset description: {}", dataset.description().unwrap());
    //let layer_count = dataset.layer_count();
    //println!("Number of layers: {layer_count}");
    let mut layer: Layer = dataset.layer_by_name(layer_name).unwrap();

    // Set the spatial filter on the layer to the BBOX
    layer.set_spatial_filter_rect(min_x, min_y, max_x, max_y);

    let mut features = Vec::new();
    for feature in layer.features() {
        let geometry = feature.geometry().unwrap().clone();
        features.push( geometry );
    }

    features
}




fn read_gpkg(gpkg_path: &str, show_features: bool, min_x: f64, min_y: f64, max_x: f64, max_y: f64) {

    let dataset = Dataset::open(gpkg_path).unwrap();
    println!("Dataset description: {}", dataset.description().unwrap());
    let layer_count = dataset.layer_count();
    println!("Number of layers: {layer_count}");
    let mut layer = dataset.layer(0).unwrap();

    // Set the spatial filter on the layer to the BBOX
    layer.set_spatial_filter_rect(min_x, min_y, max_x, max_y);


    let feature_count = layer.feature_count();
    println!("Layer name='{}', features={}", layer.name(), feature_count);

    if show_features {
        for feature in layer.features() {
            // The fid is important in cases where the vector dataset is large can you
            // need random access.
            let fid = feature.fid().unwrap_or(0);
            // Summarize the geometry
            let geometry = feature.geometry().unwrap();
            let geom_type = geometry_type_to_name(geometry.geometry_type());
            let geom_len = geometry.get_point_vec().len();
            println!("    Feature fid={fid:?}, geometry_type='{geom_type}', geometry_len={geom_len}");
            // Get all the available fields and print their values
            for field in feature.fields() {
                let name = field.0;
                let value = field.1.and_then(|f| f.into_string()).unwrap_or("".into());
                println!("      {name}={value}");
            }
        }
    }

}


fn write_gpkg(gpkg_path: &str, layer_name: &str) {

    let driver = DriverManager::get_driver_by_name("GPKG").unwrap();

    //TODO test working in memory and then saving in gpkg file ?
    let mut dataset = driver.create(gpkg_path, 0, 0, 0)
    .expect("Cannot create dataset");


    // Create layer
    let mut layer = dataset.create_layer(LayerOptions {
        name: layer_name,
        srs: Some(&SpatialRef::from_epsg(4326).unwrap()),
        ty: wkbPoint,
        ..Default::default()
    }).unwrap();

    // Define fields for the layer
    let field_def = FieldDefn::new("name", OGRFieldType::OFTString).unwrap();
    field_def.add_to_layer(&layer).unwrap();

    // Add dummy feature to the layer
    //let geometry = Geometry::from_wkt("POINT(6 10)")?;
    let ptg = point! { x: 6.0, y: 10.0 };
    let geometry = geo_point_to_gdal(&ptg).unwrap();

    let fv = FieldValue::StringValue("dkfhdskjfhds".to_string());
    layer.create_feature_fields(geometry, &[&"name"], &[fv]).unwrap();

}



fn geo_point_to_gdal(point: &Point<f64>) -> Result<Geometry, gdal::errors::GdalError> {
    let mut geom = Geometry::empty(OGRwkbGeometryType::wkbPoint)?;
    geom.add_point_2d((point.x(), point.y()));
    //geom.set_point(0, point.x(), point.y(), 0.0);
    Ok(geom)
}




//dbg!(args);


//traits
/*
pub fn notify(item: &(impl Summary + Display)) {
pub fn notify<T: Summary + Display>(item: &T) {
    fn some_function<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
*/

//fn main() -> Result<(), Box<dyn Error>> {


/*
use gdal::Dataset;
use gdal::vector::LayerAccess;
use geo::{Contains, Geometry, LineString, MinimumRotatedRect, MultiLineString, MultiPolygon, Polygon};
use std::{any::Any, path::Path};

fn main() {

    let gpkg_path = "/home/juju/geodata/gisco/CNTR_RG_03M_2024_3035.gpkg";
    //let bbox = (0.0, 0.0, 10.0, 10.0); // Bounding box: (min_x, min_y, max_x, max_y)

    // Open the GeoPackage file
    let dataset = Dataset::open(Path::new(gpkg_path)).expect("Failed to open GeoPackage");

    // Get the first vector layer
    let mut layer = dataset.layer(0).expect("Failed to access layer");
    println!("Using layer: {}", layer.name());

    // Display attribute names
    //let fields: Vec<String> = layer.defn().fields().iter().map(|f| f.name().to_string()).collect();
    //println!("Attributes: {:?}", fields);

    // Create bounding box geometry for filtering
    //let bbox_geom = geo::Rect::new((bbox.0, bbox.1).into(), (bbox.2, bbox.3).into());

    // Iterate over features
    for feature in layer.features() {

        //let nbf = feature.field_count();
        let fv = feature.field("CNTR_NAME");
        //println!("{nbf}");
        if let Ok(Some(cn)) = fv {
            println!("{:?}", cn);
        }

        // Get geometry
        if let Some(gdal_geom) = feature.geometry() {
            if let Ok(geo_geom) = gdal_geom.to_geo() {
                // Check if feature is inside the bounding box
                //if bbox_geom.contains(&geo_geom) {
                    // Extract "myatt" value
                    /*if let Some(myatt_value) = feature.field("myatt") {
                        println!("myatt: {:?}", myatt_value);
                    }*/

                    // Count vertices in geometry
                    let vertex_count = count_vertices(&geo_geom);
                    println!("Number of vertices: {}", vertex_count);

                    let ty = geo_geom.minimum_rotated_rect();
                    println!("Geom: {:?}", ty);
                //}
            }
        }
    }

}




// Function to count vertices in geometries
fn count_vertices(geometry: &Geometry<f64>) -> usize {
    match geometry {
        Geometry::Point(_) => 1,
        Geometry::LineString(ls) => ls.0.len(),
        Geometry::Polygon(poly) => poly.exterior().0.len() + poly.interiors().iter().map(|r| r.0.len()).sum::<usize>(),
        Geometry::MultiPoint(mp) => mp.0.len(),
        Geometry::MultiLineString(mls) => mls.0.iter().map(|ls| ls.0.len()).sum(),
        Geometry::MultiPolygon(mpoly) => mpoly.0.iter().map(|p| count_vertices(&Geometry::Polygon(p.clone()))).sum(),
        _ => 0,
    }
}


*/





//use rand::Rng;
//use std::cmp::Ordering;
//use std::io;



/*

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

*/



/*
//vectors
let v: Vec<i32> = Vec::new();
let v = vec![1, 2, 3];
let mut v = Vec::new();
v.push(5);
v.push(6);
v.push(7);
v.push(8);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {third}");
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

   let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

*/




/*
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
*/



/*
//typed tuples

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

*/






/*
let user2 = User {
    email: String::from("another@example.com"),
    ..user1
};*/




/*

fn main() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

*/


/*
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
assert_eq!(slice, &[2, 3]);
*/



/*
fn plus_one(x: i32) -> i32 {
    x + 1
    //return x + 1;
}*/



/*

use std::io;
use rusqlite::{Connection, Result};
use geo::{Geometry, Point, LineString, Polygon};


fn main() {

/*

    println!("Hello, World!");
    for i in 0..5 {
        let even_odd = if i % 2 == 0 {"even"} else {"odd"};
        println!("{} {}", even_odd, i);

    }
    for e in 2..3 {
        let mut ddd = "453FDS"
        println(ddd)
    }

*/




    for i in 0..3 {
        println!("{}", i)
        //std::borrow
    }

    println!("Hello, world!");
    println!("{}", FFF)
}

static FFF:i32 = 3243;

fn count_vertices(geometry: &Geometry<f64>) -> usize {
    match geometry {
        Geometry::Point(_) => 1,
        Geometry::LineString(ls) => ls.0.len(),
        Geometry::Polygon(poly) => poly.exterior().0.len() + poly.interiors().iter().map(|ring| ring.0.len()).sum::<usize>(),
        Geometry::MultiPoint(mp) => mp.0.len(),
        Geometry::MultiLineString(mls) => mls.0.iter().map(|ls| ls.0.len()).sum(),
        Geometry::MultiPolygon(mp) => mp.0.iter().map(|poly| count_vertices(&Geometry::Polygon(poly.clone()))).sum(),
        _ => 0,
    }
}


*/
