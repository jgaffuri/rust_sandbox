use rand::Rng;
use std::cmp::Ordering;
use std::io;

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
