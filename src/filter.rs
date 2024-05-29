//! Minimal example showing what we need to prove in terms of the initial
//! configuration of hosts and their statuses.

#![allow(dead_code)]

use builtin::*;
use builtin_macros::*;
use vstd::{*, prelude::*, pervasive::*};

verus! {

pub type HostID = int;

pub enum Status {
    Initial,
    Echo,
}

pub struct Host {
    pub status: Status
}

pub struct State {
    pub hosts: Set<HostID>,
    pub faulty: Set<HostID>,
    pub correct: Map<HostID, Host>,
    pub leader: nat,
}

/// What does it mean for a state to be an initial state?
pub open spec fn is_initial(s: State) -> bool {
    let hosts = s.hosts;
    let faulty = s.faulty;
    let correct = s.correct;
    let leader = s.leader;

    &&& hosts.finite()
    &&& faulty.subset_of(hosts)
    &&& correct.dom().subset_of(hosts)
    &&& forall |id: HostID| correct.dom().contains(id) ==> {
        let host = correct.index(id);
        if id == leader {
            match host.status {
                Status::Initial => true,
                _ => false,
            }
        } else {
            match host.status {
                Status::Echo => true,
                _ => false,
            }
        }
    }
}

pub proof fn one_host_with_initial_status(s: State)
    requires
        is_initial(s)
    ensures
        s.correct.dom().filter(|id: HostID| {
            let host = s.correct.index(id);
            match host.status {
                Status::Initial => true,
                _ => false,
            }
        }).len() <= 1
{
    // Get information about the state.
    let hosts = s.hosts;
    let faulty = s.faulty;
    let correct = s.correct;
    let leader = s.leader;

    // Closure that determines the filter.
    let inclusion_closure: spec_fn(HostID) -> bool =
        |h: HostID| {
            let host = correct.index(h);
            match host.status {
                Status::Initial => true,
                _ => false,
            }
        };

    if correct.dom().contains(leader as int) {
        let initial_status_hosts: Set<HostID> = s.correct.dom().filter(
            |id: HostID| {
                let host = s.correct.index(id);
                match host.status {
                    Status::Initial => true,
                    _ => false,
                }
            }
        );
        let initial_status_hosts_manual: Set<HostID> = set![leader as int];
        assert(initial_status_hosts =~= initial_status_hosts_manual);
    } else {
        let initial_status_hosts: Set<HostID> = s.correct.dom().filter(
            |id: HostID| {
                let host = s.correct.index(id);
                match host.status {
                    Status::Initial => true,
                    _ => false,
                }
            }
        );
        let initial_status_hosts_manual: Set<HostID> = set![];
        assert(initial_status_hosts =~= initial_status_hosts_manual);
    }
}

}
