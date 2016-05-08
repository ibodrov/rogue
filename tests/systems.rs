extern crate world;
extern crate specs;

use world::{systems, components, map};
use std::sync::mpsc;

#[test]
fn test_player_control() {
    let mut planner = setup_planner();

    let mut map = setup_map();
    // set up a wall
    // TODO remove hardcoded value for walls
    map[(0, 2, 0)] = 1;

    planner.world.create_now()
        .with(components::Position::new(0, 0, 0))
        .with(components::PlayerControlled::default())
        .build();

    let (sender, receiver) = mpsc::channel();
    planner.add_system(systems::PlayerControlSystem::new(receiver, map.clone()), "test", 0);

    // no action
    planner.dispatch(0.0);
    planner.wait();
    assert_position(&mut planner, (0, 0, 0));

    // move down
    move_and_check(&mut planner, &sender, systems::PlayerCommand::MoveDown, (0, 1, 0));

    // move down, hit a wall, stay on the previous spot
    move_and_check(&mut planner, &sender, systems::PlayerCommand::MoveDown, (0, 1, 0));

    // move right
    move_and_check(&mut planner, &sender, systems::PlayerCommand::MoveRight, (1, 1, 0));
}

type MyPlanner = specs::Planner<systems::TimeDelta>;

fn setup_planner() -> MyPlanner {
    let mut w = specs::World::new();
    w.register::<components::PlayerControlled>();
    w.register::<components::Position>();

    specs::Planner::new(w, 4)
}

fn setup_map() -> map::Map {
    map::Map::new((10, 10, 10), 0)
}

fn move_and_check(planner: &mut MyPlanner, sender: &chan::Sender<systems::PlayerCommand>, cmd: systems::PlayerCommand, assert_pos: (u32, u32, u32)) {
    sender.send(cmd);
    planner.dispatch(0.0);
    planner.wait();
    assert_position(planner, assert_pos);
}

fn assert_position(planner: &mut MyPlanner, position: (u32, u32, u32)) {
    planner.run0w1r(move |p: &components::Position| {
        assert_eq!(p.x, position.0);
        assert_eq!(p.y, position.1);
        assert_eq!(p.z, position.2);
    });
    planner.wait();
}
