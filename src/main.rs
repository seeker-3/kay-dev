use kay::{
    ActorSystem,
    Networking,
    Tuning,
    // TypedID
};

use std::{thread::sleep, time::Duration};

use lib::actor::{setup, CounterActorID};

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

    let world = &mut system.world();

    // system.networking_connect();
    let counter_sender1 = CounterActorID::spawn(world);
    let counter_sender2 = CounterActorID::spawn(world);
    counter_sender1.increment(Some(5), world);
    system.process_all_messages();

    loop {
        // system.networking_send_and_receive();
        counter_sender1.increment(None, world);
        system.process_all_messages();
        sleep(Duration::from_secs(1));
        counter_sender2.increment(Some(2), world);
        system.process_all_messages();
        sleep(Duration::from_secs(1));
    }
}
