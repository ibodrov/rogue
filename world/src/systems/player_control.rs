use std::sync::{mpsc, Arc, Mutex};
use specs;
use map;
use ::WorldContext;

pub enum PlayerCommand {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

pub trait ObstacleChecker: Send {
    fn check(&self, x: i32, y: i32, z: i32) -> bool;
}

pub struct MapObstactChecker {
    map: Arc<Mutex<map::Map>>,
}

impl MapObstactChecker {
    pub fn new(map: Arc<Mutex<map::Map>>) -> Self {
        MapObstactChecker {
            map: map
        }
    }
}

impl ObstacleChecker for MapObstactChecker {
    fn check(&self, x: i32, y: i32, z: i32) -> bool {
        let m = &*self.map.lock().unwrap();
        let (msx, msy, msz) = m.size();

        if x < 0 || x >= msx as i32 {
            return false;
        }

        if y < 0 || y >= msy as i32 {
            return false;
        }

        if z < 0 || z >= msz as i32 {
            return false;
        }

        let x = x as u32;
        let y = y as u32;
        let z = z as u32;

        let t = m[(x, y, z)];
        !(t == 201 || t == 205 || t == 187 || t == 186 || t == 199 || t == 217 || t == 179)
    }
}

pub struct PlayerControlSystem<C: ObstacleChecker> {
    receiver: mpsc::Receiver<PlayerCommand>,
    checker: C,
}

impl<C: ObstacleChecker> PlayerControlSystem<C> {
    pub fn new(receiver: mpsc::Receiver<PlayerCommand>, checker: C) -> Self {
        PlayerControlSystem {
            receiver: receiver,
            checker: checker,
        }
    }
}

impl<C: ObstacleChecker> specs::System<WorldContext> for PlayerControlSystem<C> {
    fn run(&mut self, arg: specs::RunArg, _: WorldContext) {
        use specs::Join;
        use components::{Position, PlayerControlled};

        let (mut pos, _) = arg.fetch(|w| (w.write::<Position>(), w.read::<PlayerControlled>()));

        match self.receiver.try_recv() {
            Ok(cmd) => {
                for p in (&mut pos).iter() {
                    let mut x = p.x as i32;
                    let mut y = p.y as i32;
                    let z = p.z as i32;

                    match cmd {
                        PlayerCommand::MoveUp => y -= 1,
                        PlayerCommand::MoveDown => y += 1,
                        PlayerCommand::MoveLeft => x -= 1,
                        PlayerCommand::MoveRight => x += 1,
                    }

                    if !self.checker.check(x, y, z) {
                        continue;
                    }

                    let x = x as u32;
                    let y = y as u32;

                    p.x = x;
                    p.y = y;
                }
            },
            Err(mpsc::TryRecvError::Empty) => (),
            Err(e) => panic!("Unhandled error while receiving player commands: {:?}", e),
        }
    }
}
