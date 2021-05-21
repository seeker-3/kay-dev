mod kay_auto;
pub use kay_auto::*;

use kay::{ActorSystem, World};

// most basic possible boilerplate
#[derive(Clone, Compact)]
pub struct BasicActor {
    id: BasicActorID,
}

// Actor must have creation method or it won't generate
impl BasicActor {
    pub fn spawn(id: BasicActorID, _: &mut World) -> Self {
        Self { id }
    }
}

// kay actor needs id field of StructName + ID
// type will be auto generated
#[derive(Clone, Compact)]
pub struct CounterActor {
    id: CounterActorID,
    number: u8,
    count: u32,
}

static mut NUMBER_COUNTERS: u8 = 0;

// methods ending in ref mut world will generate kay bindings
impl CounterActor {
    // aka new - will generate add_spawn_handler code
    pub fn spawn(id: CounterActorID, _: &mut World) -> Self {
        let number = unsafe {
            NUMBER_COUNTERS += 1;
            NUMBER_COUNTERS
        };

        Self {
            number,
            id,
            count: 0,
        }
    }
    // will generate message_handler
    pub fn increment(&mut self, value: Option<u32>, _: &mut World) {
        let value = value.unwrap_or(1);
        self.count += value;
        println!(
            "actor{}\nincrement: {}\ncount: {}\n",
            self.number, value, self.count
        );
    }
}

pub fn setup(system: &mut ActorSystem) {
    system.register::<BasicActor>();
    system.register::<CounterActor>();
    auto_setup(system);
}
