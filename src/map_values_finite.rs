//! Trying to show that the set of values in a given map (which is produced
//! from a finite set) is also a finite set.

#![allow(dead_code)]

use builtin::*;
use builtin_macros::*;
use vstd::{*, prelude::*, pervasive::*, set_lib::*, map_lib::*, relations::*};

verus! {

pub type HostID = int;

pub enum Status {
    Initial,
    Echo,
}

pub struct Host {
    pub id: HostID,
    pub status: Status,
}

pub struct State {
    pub hosts: Set<HostID>,
    pub faulty: Set<HostID>,
    pub correct: Map<HostID, Host>,
    pub leader: nat,
}

/// Function specifying what it means for a state to be an initial state.
pub open spec fn is_initial(s: State) -> bool {
    let hosts = s.hosts;
    let faulty = s.faulty;
    let correct = s.correct;
    let leader = s.leader;

    &&& hosts.finite()
    &&& faulty.finite()
    &&& faulty.subset_of(hosts)
    &&& correct.dom().subset_of(hosts)
    &&& correct =~= hosts.difference(faulty).mk_map(|id: HostID|
            Host {
                id,
                status: if id == leader {
                    Status::Initial
                } else {
                    Status::Echo
                },
            }
        )
}

pub proof fn lemma_recursive_show_length<A, B>(
    domain: Set<A>,
    range: Set<B>,
    map: Map<A, B>,
    f: spec_fn(A) -> B,
)
    requires
        domain.finite(),
        domain =~= map.dom(),
        range =~= map.values(),
        map =~= domain.mk_map(f),
        injective(f),
    ensures
        domain.len() == range.len()
    decreases
        domain.len(),
        range.len(),
        map.len(),
{
    // TODO: Fix this later.
    assume(false);

    if range =~= Set::empty() {
    } else if domain =~= Set::empty() {
    } else if map.is_empty() {
    } else {
        let a: A = domain.choose();
        assert(domain.contains(a));

        let b: B = f(a);
        assert(range.contains(b));

        assert(map.dom().contains(a));
        assert(map.values().contains(b));
        assert(map.index(a) == b);

        let new_domain = domain.remove(a);
        let new_range = range.remove(b);
        let new_map = map.remove(a);

        lemma_recursive_show_length(new_domain, new_range, new_map, f);
    }
}

/// Show that the closure we use for constructing the set of correct hosts is
/// in fact not injective.
pub proof fn injective_closure_correct(leader: HostID)
    ensures
        !injective(|id: HostID| {
                Host {
                    id: 0int,
                    status: if id == leader {
                        Status::Initial
                    } else {
                        Status::Echo
                    }
                }
            }
        )
{
    let f: spec_fn(HostID) -> Host = |id: HostID| {
        Host {
            id: 0int,
            status: if id == leader {
                Status::Initial
            } else {
                Status::Echo
            }
        }
    };

    let first: HostID = leader + 1;
    let second: HostID = leader + 2;

    assert(first != leader);
    assert(second != leader);
    assert(first != second);

    let first_image = f(first);
    let second_image = f(second);

    assert(first_image == second_image);
}

pub proof fn correct_values_finite(s: State)
    requires
        is_initial(s)
    ensures
        s.correct.values().finite(),
        s.correct.dom().len() == s.correct.values().len()
{
    // Asserting that this set is finite is relatively simple it seems.
    assert(s.correct.dom().finite());
    lemma_values_finite(s.correct);

    // The closure that is used for constructing this set.
    let closure = |id: HostID|
            Host {
                id,
                status: if id == s.leader {
                    Status::Initial
                } else {
                    Status::Echo
                },
            };

    // There has to be a recursive proof here.
    // lemma_recursive_show_length(s.correct, closure);

    // TODO: Fix this later.
    assume(s.correct.dom().len() == s.correct.values().len());
}

}
