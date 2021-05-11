use kay::{
    ActorSystem,
    Networking,
    Tuning,
    // TypedID
};

use lib::actor::{setup, ActorWrapperID, BasicActorID};

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

    setup(&mut system);

    let mut world = system.world();

    system.networking_connect();
    system.networking_send_and_receive();

    let id1 = BasicActorID::spawn(&mut world);

    let actor1 = ActorWrapperID::spawn(id1, &mut world);
    let actor2 = ActorWrapperID::spawn(id1, &mut world);
    actor1.increment(&mut world);
    actor2.increment(&mut world);

    system.process_all_messages();
}
