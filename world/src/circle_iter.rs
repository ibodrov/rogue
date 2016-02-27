///
/// Based on https://github.com/bluss/rpasha
///

/// \....|..../
/// .\.5.|.3./.
/// ..\..|../..
/// .6.\.|./.4.
/// ....\|/....
/// -----+-----
/// ..../|\....
/// .8./.|.\.2.
/// ../..|..\..
/// ./.7.|.1.\.
/// /....|....\
const CIRCLE_OCTANTS: [(i32, i32, bool); 8] = [
    ( 1,   1, true),
    ( 1,   1, false),
    ( 1,  -1, true),
    ( 1,  -1, false),
    (-1,  -1, true),
    (-1,  -1, false),
    (-1,   1, true),
    (-1,   1, false),
];

#[derive(Debug)]
pub struct CircleIter {
    octant: usize,
    radius: i32,
    r: i32,
    x: i32,
}

/// x, y (near, center, far)
pub type CircleItem = (i32, i32, (f32, f32, f32));

impl CircleIter {
    pub fn new(radius: i32) -> Self {
        CircleIter {
            octant: 0,
            radius: radius,
            r: 0,
            x: 0,
        }
    }

    fn next_octant(&mut self) {
        self.octant += 1;
        if self.octant >= CIRCLE_OCTANTS.len() {
            self.octant = 0;
        }
    }

    pub fn octant(&self) -> usize {
        self.octant
    }
}

impl Iterator for CircleIter {
    type Item = CircleItem;

    fn next(&mut self) -> Option<CircleItem> {
        const PERMISSIVE_RADIUS: f32 = 0.33;

        if self.r == 0 {
            self.r += 1;
            return Some((0, 0, (0.0, 0.0, 0.0)));
        }

        if self.x > self.r {
            self.x = 0;
            self.r += 1;
        }

        if self.r > self.radius {
            self.next_octant();
            self.x = 0;
            self.r = 1;
            if self.octant == 0 {
                return None;
            }
            return self.next();
        }

        let (qx, qy, vert) = CIRCLE_OCTANTS[self.octant];

        let (a, b) = if vert {
            (self.x * qx, self.r * qy)
        } else {
            (self.r * qx, self.x * qy)
        };

        if (a as f32).hypot(b as f32) >= PERMISSIVE_RADIUS + self.radius as f32 {
            self.x += 1;
            return self.next();
        }

        let angle_alloc = 1.0 / ((self.r + 1) as f32);
        let near = (self.x as f32) * angle_alloc;
        let center = near + 0.5 * angle_alloc;
        let far = near + angle_alloc;

        self.x += 1;
        Some((a, b, (near, center, far)))
    }
}

#[test]
fn test_iter() {
    let w = 16;
    let h = 16;

    let idx = |x: i32, y: i32| (x + y * 16) as usize;

    let base_x = w / 2;
    let base_y = h / 2;

    let mut data = (0..w * h).map(|_| '.').collect::<Vec<_>>();

    let ci = CircleIter::new(5);
    for (x, y, _) in ci {
        let n = idx(base_x + x, base_y + y);
        if data[n] == '*' {
            data[n] = '0';
        } else {
            data[n] = '*';
        }
    }

    for y in 0..h {
        for x in 0..w {
            let n = idx(x, y);
            print!("{}", data[n]);
        }
        println!("");
    }
}
