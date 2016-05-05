#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

extern crate specs;
#[macro_use]
extern crate chan;
extern crate time;

pub mod map;
pub mod tile;
pub mod components;
pub mod systems;

pub use systems::PlayerCommand;

pub type TimeDelta = f64;

pub struct World {
    planner: specs::Planner<TimeDelta>,
    map: map::Map,
    player_commands: chan::Sender<PlayerCommand>,
    last_tick: f64,
}

impl Default for World {
    fn default() -> Self {
        let map = map::Map::new((100, 100, 3), 0);
        let (cmd_sender, cmd_receiver) = chan::async();

        let planner = {
            let mut w = specs::World::new();
            w.register::<components::Position>();
            w.register::<components::Visible>();
            w.register::<components::PlayerControlled>();

            let mut p = specs::Planner::new(w, 4);
            p.add_system(systems::PlayerControlSystem::new(cmd_receiver, map.clone()), "player-control", 100);

            p
        };

        World {
            map: map,
            player_commands: cmd_sender,
            planner: planner,

            // TODO avoid a huge time delta on the first "tick()"
            last_tick: 0.0,
        }
    }
}

impl World {
    pub fn tick(&mut self) {
        let dt = time::precise_time_s() - self.last_tick;
        self.planner.dispatch(dt);
    }

    pub fn send_player_command(&mut self, cmd: PlayerCommand) {
        self.player_commands.send(cmd);
    }

    pub fn map(&self) -> &map::Map {
        &self.map
    }
}
