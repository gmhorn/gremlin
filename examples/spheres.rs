use gremlin::{camera::Perspective, film::RGBFilm, material::Lambertian, scene::Scene};

fn main() {
    let mut img = RGBFilm::new(800, 600);
    let mut cam = Perspective::new(img.aspect_ratio(), 55.0);
    cam.move_to([1.0, 0.5, 1.5]);

    let mut scene = Scene::default();
    // scene.add_primitive(Sphere::new([-0.5, 0.0, -1.0], 0.5), Lambertian::new(RGB::from([0.5, 0.5, 0.5])));
}

fn render() {}
