// use std::ops::{Deref, DerefMut};

mod film;
pub use film::*;

mod pixel;
pub use pixel::*;

// #[derive(Debug, Clone, Copy)]
// pub struct Pixel {
//     pub x: f32,
//     pub y: f32,
//     pub z: f32,
//     pub n: u32,
// }

// impl Pixel {
//     pub fn new() -> Self {
//         Self{ x: 0.0, y: 0.0, z: 0.0, n: 0}
//     }
// }

// pub struct Film {
//     width: u32,
//     height: u32,
//     pixels: Vec<Pixel>,
// }

// impl Film {
//     pub fn new(width: u32, height: u32) -> Self {
//         let pixels = vec![Pixel::new(); (width * height) as usize];
//         Self{ width, height, pixels }
//     }

//     // pub fn values(self) -> () {
//     //     self.pixels.into_iter()
//     // }
// }

// impl IntoIterator for Film {
//     type Item = Pixel;
//     type IntoIter = std::vec::IntoIter<Pixel>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.pixels.into_iter()
//     }
// }

// impl Deref for Film {
//     type Target = [Pixel];

//     fn deref(&self) -> &Self::Target {
//         &self.pixels
//     }
// }

// impl DerefMut for Film {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.pixels
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn foo() {
//         let f = Film::new(300, 200);
//         for p in f {
//         }
//     }
// }