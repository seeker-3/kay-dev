use kay::World;

#[derive(Clone, Compact)]
pub struct BasicActor {
    id: BasicActorID,
    count: u8,
}

impl BasicActor {
    pub fn spawn(id: BasicActorID, _: &mut World) -> BasicActor {
        println!("spawned!!");
        BasicActor { id, count: 0 }
    }
    pub fn increment(&mut self, _: &mut World) {
        self.count += 1;
        println!("incremented: {}", self.count);
    }
}

mod kay_auto;
pub use kay_auto::*;
