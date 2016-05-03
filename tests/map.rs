extern crate world;

use world::map;

#[test]
fn test_map_update() {
    let mut m = map::Map::new((10, 10, 10), 0);
    assert_eq!(m[(0, 0, 0)], 0);

    let c = map::MapChunk::new((1, 1, 1), (5, 5, 5), 1);
    m.update(c);
    assert_eq!(m[(0, 0, 0)], 0);
    assert_eq!(m[(1, 1, 1)], 0);

    m.apply_updates();
    assert_eq!(m[(1, 1, 1)], 1);
    assert_eq!(m[(5, 5, 5)], 1);
    assert_eq!(m[(6, 6, 0)], 0);
}
