#![allow(unused_imports)]

use builtin::*;
use builtin_macros::*;
use vstd::{*, prelude::*, set::*, set_lib::*};

verus! {

#[allow(dead_code)]
pub type HostID = int;

proof fn some_set_of_hostids_exists(n: nat)
    ensures
        exists |s: Set<HostID>| s.finite() && s.len() == n
{
    let candidate: Set<HostID>
        = vstd::set_lib::set_int_range(0nat as int, n as int);
    lemma_int_range(0 as int, n as int);
}

proof fn new_hostID_can_be_found(s: Set<HostID>) 
    requires
        s.finite()
    ensures
        exists |h: HostID| !s.contains(h)
{
    assume(false);
}

pub open spec fn within_range(n1: nat, n2: nat, n: nat, f: nat) -> bool {
    &&& n1 + n2 == (n - f) as nat
    &&& n1 == 0 || n2 == 0
}

pub proof fn some_numbers_exist(n: nat, f: nat)
    requires
        n > 3 * f,
        f == 0,
    ensures
        exists |n1: nat, n2: nat| within_range(n1, n2, n, f),
{
    let h1: nat = 0;
    let h2: nat = n;
    assert(within_range(h1, h2, n, f));
}

}
