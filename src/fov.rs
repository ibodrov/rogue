///
/// Based on https://github.com/bluss/rpasha
///

use std::vec::Vec;
use circle_iter::CircleIter;

pub struct RPAPartialShadowcasting<F> {
    check_fn: F,
    iter: CircleIter,
    last_octant: usize,
    obstructions: Vec<(f32, f32, f32)>,
}

impl<F> RPAPartialShadowcasting<F> where F: FnMut(i32, i32) -> f32 {

    pub fn new(radius: i32, check_fn: F) -> Self {
        RPAPartialShadowcasting {
            check_fn: check_fn,
            iter: CircleIter::new(radius),
            last_octant: 0,
            obstructions: Vec::new(),
        }
    }
}

impl<F> Iterator for RPAPartialShadowcasting<F> where F: FnMut(i32, i32) -> f32 {
    type Item = (i32, i32, f32);

    fn next(&mut self) -> Option<(i32, i32, f32)> {
        let (a, b, (near, center, far)) = match self.iter.next() {
            Some(x) => x,
            _ => return None,
        };

        let iter_octant = self.iter.octant();
        if self.last_octant != iter_octant {
            self.obstructions.clear();
            self.last_octant = iter_octant;
        }

        let mut opacity = 0.0f32;
        let mut near_vis = true;
        let mut center_vis = true;
        let mut far_vis = true;

        for &(near_obs, far_obs, opc_obs) in self.obstructions.iter() {
            fn angle_in(angle: f32, start: f32, end: f32) -> bool {
                start <= angle && angle <= end
            }

            fn visible_when(center: bool, near: bool, far: bool) -> bool {
                center && (near || far)
            }

            near_vis = near_vis && !angle_in(near, near_obs, far_obs);
            center_vis = center_vis && !angle_in(center, near_obs, far_obs);
            far_vis = far_vis && !angle_in(far, near_obs, far_obs);

            if !center_vis {
                opacity = opacity.max(0.5 * opc_obs);
            }

            if !visible_when(center_vis, near_vis, far_vis) {
                opacity = opacity.max(opc_obs);
            }

            if opacity >= 1.0 {
                break;
            }
        }

        let mut opc_here = (self.check_fn)(a, b);
        opc_here = opacity + opc_here;
        if opc_here > 0.0 {
            self.obstructions.push((near, far, opc_here));
        }
        opacity = opacity.min(1.0);

        Some((a, b, opacity))
    }
}

#[test]
fn test_iter() {
    let w = 16i32;
    let h = 16i32;

    let idx = |x: i32, y: i32| (x + y * 16) as usize;

    let base_x = w / 2;
    let base_y = h / 2;

    let data: Vec<u8> = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0,
        0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    assert_eq!(data.len(), (w * h) as usize);
    let mut result = data.clone();

    let it = RPAPartialShadowcasting::new(5, |x, y| {
        let n = idx(base_x + x, base_y + y);
        let t = data[n];
        if t == 0 { 0.0 } else { 1.0 }
    });

    for (x, y, o) in it {
        let n = idx(base_x + x, base_y + y);
        if o < 1.0 {
            result[n] = 2;
        } else {
            result[n] = 0;
        }
    }

    for y in 0..h {
        for x in 0..w {
            let n = idx(x, y);
            print!("{}", result[n]);
        }
        println!("");
    }
}
