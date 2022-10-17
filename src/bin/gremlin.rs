use gremlin::geo::{Vec3, Point3};

fn main() {
    println!("Hello from gremlin!");

    let p1 = Point3::splat(1.0);
    let p2 = Point3::new(5.0, 6.0, 7.0);

    let v1 = Vec3::new(-1.0, -2.0, -3.0);
    let point_plus_point = p1 + p2;
    let point_plus_vec = p1 + v1;

    println!("{:?} + {:?} = {:?}", p1, p2, point_plus_point);
    println!("{:?} + {:?} = {:?}", p1, v1, point_plus_vec);
}