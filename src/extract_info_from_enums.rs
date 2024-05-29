//! Can we extract information from enumerated types nicely?

#![allow(dead_code)]

use builtin::*;
use builtin_macros::*;
use vstd::{*, prelude::*, pervasive::*, set_lib::*, map_lib::*, relations::*};

verus! {

pub type HostID = int;

pub enum Value {
    Zero,
    One
}

pub enum Message {
    Initial(Value),
    Echo(Value),
    Ready(Value),
}

pub struct Packet {
    pub src: HostID,
    pub dst: HostID,
    pub msg: Message,
}

pub enum Step {
    Initial(HostID, Value),
    Echo(HostID, Packet, Value),
    Ready(HostID, Packet, Value),
    Decide(HostID, Packet, Value),
    Fault(Set<Packet>),
}

pub proof fn extract_information_about_step(
    s: Step
)
    requires
        s matches Step::Initial(id, v)
    ensures
        true
{
    let outer_id: HostID = match s {
        Step::Initial(id, v) => id,
        _ => arbitrary(),
    };
    assert({
        match s {
            Step::Initial(id, v) => id == outer_id,
            _ => false,
        }
    });
}

}
