use gremlin::geo::{Vec3, Point3};

fn main() {
    println!("Hello from gremlin!");

    let v = Vec3::splat(2.0);
    let p = Point3::splat(12.0);

    println!("Got a vector {:?} and point {:?}", v, p);
}