use gremlin::geo::{Point, Vector};

fn main() {
    println!("Hello from gremlin!");

    let p1 = Point::splat(1.0);
    let p2 = Point::new(5.0, 6.0, 7.0);

    let v1 = Vector::new(-1.0, -2.0, -3.0);
    let point_plus_point = p1 + p2;
    let point_plus_vec = p1 + v1;

    println!("{:?} + {:?} = {:?}", p1, p2, point_plus_point);
    println!("{:?} + {:?} = {:?}", p1, v1, point_plus_vec);
}
