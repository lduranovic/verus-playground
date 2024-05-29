#![allow(dead_code)]

use builtin::*;
use builtin_macros::*;
use vstd::prelude::*;

verus! {

pub open spec fn populate_set(i: nat, s: Set<int>) -> Set<int> {
    let initial_set: Set<int> = s;
    if i == 0nat {
        initial_set.insert(0)
    } else if i == 1nat {
        initial_set.insert(1)
    } else {
        initial_set.insert(2)
    }
}

pub proof fn prove_fact(i: nat) 
    requires
        i == 1nat
    ensures
        populate_set(i, set![]) =~= set![1int]
{
}

}
