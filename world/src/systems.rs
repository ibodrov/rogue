use std::sync::mpsc;
use specs;
use components;
use map;

pub enum PlayerCommand {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

pub struct PlayerControlSystem {
    receiver: mpsc::Receiver<PlayerCommand>,
}

impl PlayerControlSystem {
    pub fn new(receiver: mpsc::Receiver<PlayerCommand>) -> Self {
        PlayerControlSystem {
            receiver: receiver,
        }
    }
}

impl specs::System<super::WorldContext> for PlayerControlSystem {
    fn run(&mut self, arg: specs::RunArg, ctx: super::WorldContext) {
        use specs::Join;

        // w/a specs limitations, we must fetch components whether we need them or not
        let (mut pos, _) = arg.fetch(|w| (w.write::<components::Position>(), w.read::<components::PlayerControlled>()));

        match self.receiver.try_recv() {
            Ok(cmd) => {
                let map = ctx.map;
                let (map_size_x, map_size_y, _) = map.size();
                let (map_size_x, map_size_y) = (50, 50);

                for p in (&mut pos).iter() {
                    let mut x = p.x as i32;
                    let mut y = p.y as i32;

                    match cmd {
                        PlayerCommand::MoveUp => y -= 1,
                        PlayerCommand::MoveDown => y += 1,
                        PlayerCommand::MoveLeft => x -= 1,
                        PlayerCommand::MoveRight => x += 1,
                    }

                    if x < 0 || x >= map_size_x as i32 {
                        continue;
                    }

                    if y < 0 || y >= map_size_y as i32 {
                        continue;
                    }

                    let x = x as u32;
                    let y = y as u32;

                    let t = map[(x, y, p.z)];
                    if t == 1 {
                        continue;
                    }

                    p.x = x;
                    p.y = y;
                }
            },
            Err(mpsc::TryRecvError::Empty) => (),
            Err(e) => panic!("Unhandled error while receiving player commands: {:?}", e),
        }
    }
}
