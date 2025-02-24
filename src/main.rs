use gdal::Dataset;
use gdal::vector::LayerAccess;
use geo::{Geometry, LineString, MultiLineString, MultiPolygon, Polygon, Contains};
use std::path::Path;


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
