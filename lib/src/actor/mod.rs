mod kay_auto;
pub use kay_auto::*;

use kay::{ActorSystem, World};

#[derive(Clone, Compact)]
pub struct BasicActor {
    id: BasicActorID,
    count: u8,
}

impl BasicActor {
    pub fn spawn(id: BasicActorID, _: &mut World) -> Self {
        println!("spawned \"basic actor\"");
        BasicActor { id, count: 0 }
    }
    pub fn increment(&mut self, _: &mut World) {
        self.count += 1;
        self.log_info()
    }
    pub fn increment_by(&mut self, value: u8, _: &mut World) {
        self.count += value;
        self.log_info()
    }
    fn log_info(&self) {
        println!("incremented: {}", self.count);
    }
}

#[derive(Clone, Compact)]
pub struct ActorWrapper {
    id: ActorWrapperID,
    actor_id: BasicActorID,
}

impl ActorWrapper {
    pub fn spawn(id: ActorWrapperID, actor_id: BasicActorID, _: &mut World) -> Self {
        println!("spawned \"another \"actor");
        ActorWrapper { id, actor_id }
    }
    pub fn increment(&mut self, world: &mut World) {
        self.actor_id.increment(world);
    }
}

pub fn setup(system: &mut ActorSystem) {
    system.register::<BasicActor>();
    system.register::<ActorWrapper>();
    auto_setup(system);
}
