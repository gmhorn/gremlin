use gremlin::{camera::ThinLens, film::RGBFilm};

fn main() {
    let img = RGBFilm::new(800, 600);
    let _cam = ThinLens::builder(img.dimensions())
        .move_to([1.0, 0.5, 1.5])
        .look_at([0.0, 0.0, -1.0])
        .fov(55.0)
        .aperture(0.25)
        .auto_focus()
        .build();

    // let mut scene = Scene::default();
    // scene.add_primitive(Sphere::new([-0.5, 0.0, -1.0], 0.5), Lambertian::new(RGB::from([0.5, 0.5, 0.5])));
}
