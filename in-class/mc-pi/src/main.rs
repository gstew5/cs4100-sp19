use std::io;
use rand::prelude::*;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn dist(&self, other: &Point) -> f64 {
        let xdist = self.x - other.x;
        let ydist = self.y - other.y;
        ((xdist*xdist) + (ydist*ydist)).sqrt()
    }
}

fn main() {
    let center = Point{x: 0.5, y: 0.5};
    let mut rng = rand::thread_rng();
    let mut in_circle = 0;
    let n = 1000;

    for _i in 0..n {
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();
        let p = Point{x: x, y: y};
        println!("{:?}", p);
        if p.dist(&center) < 0.5 {
            in_circle += 1
        }
    }

    println!("in = {}", in_circle);
    println!("pi = {}", 4.0 * (in_circle as f64 / n as f64));
    println!("Hello, world!");
}
