#![allow(dead_code)]

use builtin::*;
use builtin_macros::*;
use vstd::prelude::*;

verus! {

pub open spec fn check_if_two(i: int) -> bool {
    true
}

}

// TODO: This macros definitely needs more work. I will start off with some
// simpler ones for now.
#[macro_export]
macro_rules! prove_two_numbers_equal_after_adding_one {
    ($x:literal, $y:literal, $type: ty) => {
        verus! {
            pub proof fn show_equality($x: $type, $y: $type) 
                requires $x == $y,
                ensures $x + (1 as $type) == $y + (1 as $type),
            {
            }
        }
    }
}
