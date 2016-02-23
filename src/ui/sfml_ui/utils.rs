extern crate sfml;

use sfml::system::Vector2f;

pub fn vector2f_to_pair_i32(v: &Vector2f) -> (i32, i32) {
    let Vector2f { x, y } = *v;
    (x as i32, y as i32)
}
