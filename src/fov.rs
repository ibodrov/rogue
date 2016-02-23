///
/// Based on https://github.com/bluss/rpasha
///

use std::vec::Vec;

use world::map;
use circle_iter::CircleIter;

pub struct FOV {
    data: Vec<f32>,
    map_size: (u32, u32, u32),
}

impl FOV {
    pub fn new(map: &map::Map<u8>, start_x: u32, start_y: u32, start_level: u32, r: u32) -> Self {
        FOV {
            data: FOV::calculate(map, start_x, start_y, start_level, r),
            map_size: map.size(),
        }
    }

    fn calculate(map: &map::Map<u8>, start_x: u32, start_y: u32, start_level: u32, r: u32) -> Vec<f32> {
        let (map_w, map_h, _) = map.size();

        let check = |x: i32, y: i32| {
            const WALL: f32 = 1.0;
            const NOTHING: f32 = 0.0;

            let map_x = ((start_x as i32) + x) as u32;
            let map_y = ((start_y as i32) + y) as u32;
            if map_x >= map_w || map_y >= map_h {
                return WALL;
            }

            if *map.get_at(map_x, map_y, start_level) == 1 { WALL } else { NOTHING }
        };

        let mut result = (0..map_w * map_h).map(|_| 1.0).collect::<Vec<f32>>();

        let max_n = (map_w * map_h) as usize;
        let it = RPAPartialShadowcasting::new(r as i32, check);
        for (x, y, o) in it {
            let n = ((start_x as i32 + x) + (start_y as i32 + y) * map_w as i32) as usize;
            if n >= max_n {
                continue;
            }
            result[n] = o;
        }

        result
    }

    pub fn get_at(&self, x: u32, y: u32) -> f32 {
        let n = (x + y * self.map_size.0) as usize;
        self.data[n]
    }
}

/// Restrictive Precise Angle Shadowcasting

struct RPAPartialShadowcasting<F> {
    check_fn: F,
    iter: CircleIter,
    last_octant: usize,
    obstructions: Vec<(f32, f32, f32)>,
}

impl<F> RPAPartialShadowcasting<F> where F: FnMut(i32, i32) -> f32 {

    fn new(radius: i32, check_fn: F) -> Self {
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

    let base_x = w / 2 - 1;
    let base_y = h / 2 - 1;

    let data: Vec<u8> = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 1,10, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    assert_eq!(data.len(), (w * h) as usize);
    let mut result = Vec::with_capacity((w * h) as usize);
    result.resize((w * h) as usize, 1.0);

    let it = RPAPartialShadowcasting::new(5, |x, y| {
        let n = idx(base_x + x, base_y + y);
        let t = data[n];
        t as f32 * 0.1
    });

    for (x, y, o) in it {
        let n = idx(base_x + x, base_y + y);
        result[n] = o;
    }

    for y in 0..h {
        for x in 0..w {
            let n = idx(x, y);
            let v = result[n];
            let c = if v <= 0.0 { ' ' } else if v <= 0.1 { '.' } else if v <= 0.5 { '*' } else if v <= 0.75 { '%' } else if v <= 1.0 { '#' } else { '0' };
            print!("{}", c);
        }
        println!("");
    }
}
