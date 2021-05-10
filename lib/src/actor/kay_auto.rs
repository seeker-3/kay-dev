//! This is all auto-generated. Do not touch.
#![rustfmt::skip]
#[allow(unused_imports)]
use kay::{ActorSystem, TypedID, RawID, Fate, Actor, TraitIDFrom, ActorOrActorTrait};
#[allow(unused_imports)]
use super::*;



impl Actor for BasicActor {
    type ID = BasicActorID;

    fn id(&self) -> Self::ID {
        self.id
    }
    unsafe fn set_id(&mut self, id: RawID) {
        self.id = Self::ID::from_raw(id);
    }
}


pub struct BasicActorID {
    _raw_id: RawID
}

impl Copy for BasicActorID {}
impl Clone for BasicActorID { fn clone(&self) -> Self { *self } }
impl ::std::fmt::Debug for BasicActorID {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "BasicActorID({:?})", self._raw_id)
    }
}
impl ::std::hash::Hash for BasicActorID {
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
        self._raw_id.hash(state);
    }
}
impl PartialEq for BasicActorID {
    fn eq(&self, other: &BasicActorID) -> bool {
        self._raw_id == other._raw_id
    }
}
impl Eq for BasicActorID {}

impl TypedID for BasicActorID {
    type Target = BasicActor;

    fn from_raw(id: RawID) -> Self {
        BasicActorID { _raw_id: id }
    }

    fn as_raw(&self) -> RawID {
        self._raw_id
    }
}

impl BasicActorID {
    pub fn spawn(world: &mut World) -> Self {
        let id = BasicActorID::from_raw(world.allocate_instance_id::<BasicActor>());
        let swarm = world.local_broadcast::<BasicActor>();
        world.send(swarm, MSG_BasicActor_spawn(id, ));
        id
    }
    
    pub fn increment(self, world: &mut World) {
        world.send(self.as_raw(), MSG_BasicActor_increment());
    }
}

#[derive(Copy, Clone)] #[allow(non_camel_case_types)]
struct MSG_BasicActor_spawn(pub BasicActorID, );
#[derive(Copy, Clone)] #[allow(non_camel_case_types)]
struct MSG_BasicActor_increment();


#[allow(unused_variables)]
#[allow(unused_mut)]
pub fn auto_setup(system: &mut ActorSystem) {
    
    
    system.add_spawner::<BasicActor, _, _>(
        |&MSG_BasicActor_spawn(id, ), world| {
            BasicActor::spawn(id, world)
        }, false
    );
    
    system.add_handler::<BasicActor, _, _>(
        |&MSG_BasicActor_increment(), instance, world| {
            instance.increment(world); Fate::Live
        }, false
    );
}