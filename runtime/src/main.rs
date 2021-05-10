use kay::{ActorSystem, Networking, Tuning};

use lib::actor::{auto_setup, BasicActor, BasicActorID};

fn main() {
    let mut system = ActorSystem::new(
        Networking::new(
            0,
            vec!["localhost:9999".to_owned(), "wsclient".to_owned()],
            50_000,
            30,
            10,
        ),
        Tuning::default(),
    );

    system.register::<BasicActor>();
    auto_setup(&mut system);

    let mut world = system.world();

    let actor = BasicActorID::spawn(&mut world);

    system.process_all_messages();

    actor.increment(&mut world);

    system.process_all_messages();
}
