// use kay::{Actor, ActorSystem, Networking, RawID, Tuning, TypedID};

// #[derive(Clone, Copy, Hash, Debug)]
// struct ActorID {
//     id: RawID,
// }

// impl TypedID for ActorID {
//     type Target = BasicActor;
//     fn as_raw(&self) -> RawID {
//         self.id
//     }
//     fn from_raw(id: RawID) -> Self {
//         ActorID { id }
//     }
// }

// #[derive(Clone, Copy)]
// struct BasicActor {
//     id: ActorID,
// }

// impl Actor for BasicActor {
//     type ID = ActorID;
//     fn id(&self) -> <Self as Actor>::ID {
//         self.id
//     }
//     unsafe fn set_id(&mut self, raw_id: RawID) {
//         self.id = Self::ID::from_raw(raw_id);
//     }
// }