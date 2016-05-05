use specs;
use chan;
use components;
use map;

pub type TimeDelta = f64;

pub enum PlayerCommand {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

pub struct PlayerControlSystem {
    receiver: chan::Receiver<PlayerCommand>,
    map: map::Map
}

impl PlayerControlSystem {
    pub fn new(receiver: chan::Receiver<PlayerCommand>, map: map::Map) -> Self {
        PlayerControlSystem {
            receiver: receiver,
            map: map,
        }
    }
}

impl specs::System<TimeDelta> for PlayerControlSystem {
    fn run(&mut self, arg: specs::RunArg, _: TimeDelta) {
        use specs::Join;

        let (mut pos, _) = arg.fetch(|w| (w.write::<components::Position>(), w.read::<components::PlayerControlled>()));

        let recv = &mut self.receiver;
        loop {
            chan_select! {
                default => {
                    break;
                },

                recv.recv() -> cmd => {
                    if let Some(cmd) = cmd {
                        let map = &mut self.map;
                        map.apply_updates();

                        for p in (&mut pos).iter() {
                            let mut x = p.x;
                            let mut y = p.y;

                            match cmd {
                                PlayerCommand::MoveUp => y -= 1,
                                PlayerCommand::MoveDown => y += 1,
                                PlayerCommand::MoveLeft => x -= 1,
                                PlayerCommand::MoveRight => x += 1,
                            }

                            let t = map[(x, y, p.z)];
                            if t == 1 {
                                continue;
                            }

                            p.x = x;
                            p.y = y;
                        }
                    }
                },
            }
        }
    }
}
