use builtin::*;
use builtin_macros::*;
use vstd::prelude::*;

verus! {

pub open spec fn max_two_numbers(a: nat, b: nat) -> nat {
    if a >= b {
        a
    } else {
        b
    }
}

#[allow(dead_code)]
fn exec_max_two_numbers(a: u32, b: u32) -> (maximum: u32)
    ensures
        maximum == max_two_numbers(a as nat, b as nat)
{
    if a >= b {
        a
    } else {
        b
    }
}

}
