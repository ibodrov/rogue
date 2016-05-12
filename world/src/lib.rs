#[macro_use]
extern crate log;
extern crate specs;
extern crate time;

pub mod map;
pub mod tile;
pub mod components;
pub mod systems;

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
pub use systems::player_control::PlayerCommand;

pub type TimeDelta = f64;
pub type MapHolder = Arc<Mutex<map::Map>>;

#[derive(Clone)]
pub struct WorldContext {
    time_delta: TimeDelta,
    map: MapHolder,
}

impl WorldContext {
    pub fn new(time_delta: f64, map: MapHolder) -> Self {
        WorldContext {
            time_delta: time_delta,
            map: map,
        }
    }
}

pub struct World {
    planner: specs::Planner<WorldContext>,
    map: MapHolder,
    player_commands: mpsc::Sender<PlayerCommand>,
    last_render: systems::render::RenderedViewHolder,
    render_view: systems::render::ViewHolder,
    last_tick: f64,
}

impl Default for World {
    fn default() -> Self {
        let map = Arc::new(Mutex::new(map::load_from_csv("assets/test_map.csv", (50, 50, 1))));
        let checker = systems::player_control::MapObstactChecker::new(map.clone());

        let (cmd_sender, cmd_receiver) = mpsc::channel();

        let last_render_holder = Arc::new(Mutex::new(None));
        let render_view_holder = Arc::new(Mutex::new(systems::render::View::default()));

        let planner = {
            let mut w = specs::World::new();
            w.register::<components::Position>();
            w.register::<components::Visible>();
            w.register::<components::PlayerControlled>();

            // Add a controllable entity
            w.create_now()
                .with(components::Position::new(10, 10, 0))
                .with(components::PlayerControlled::default())
                .with(components::Visible::default())
                .build();

            let mut p = specs::Planner::new(w, 4);
            p.add_system(systems::player_control::PlayerControlSystem::new(cmd_receiver, checker), "player-control", 100);
            p.add_system(systems::render::RenderingSystem::new(last_render_holder.clone(),
                                                               render_view_holder.clone()), "rendering", 200);

            p
        };

        World {
            map: map,
            player_commands: cmd_sender,
            last_render: last_render_holder,
            render_view: render_view_holder,
            planner: planner,
            last_tick: time::precise_time_s(),
        }
    }
}

impl World {
    pub fn tick(&mut self) {
        let dt = time::precise_time_s() - self.last_tick;
        let ctx = WorldContext::new(dt, self.map.clone());
        self.planner.dispatch(ctx);
    }

    pub fn send_player_command(&mut self, cmd: PlayerCommand) {
        match self.player_commands.send(cmd) {
            Ok(_) => (),
            Err(e) => panic!("Unhandled error while sending player commands: {:?}", e),
        }
    }

    pub fn last_render(&self) -> &systems::render::RenderedViewHolder {
        &self.last_render
    }

    pub fn render_view(&self) -> &systems::render::ViewHolder {
        &self.render_view
    }

    pub fn map(&self) -> &MapHolder {
        &self.map
    }

    pub fn map_mut(&mut self) -> &mut MapHolder {
        &mut self.map
    }
}
