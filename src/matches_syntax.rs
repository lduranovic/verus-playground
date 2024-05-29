#![allow(dead_code)]

use builtin::*;
use builtin_macros::*;
use vstd::prelude::*;

verus! {

pub enum EnumeratedType {
    First(i32),
    Second(i32),
}

pub open spec fn sum_of_two_enumerated_types(
    first: EnumeratedType,
    second: EnumeratedType
) -> bool
{
    &&& first matches EnumeratedType::First(i)
    &&& second matches EnumeratedType::Second(i)
}

fn prove_sum_of_two_enumerated_types()
{
    let first = EnumeratedType::First(2i32);
    let second = EnumeratedType::Second(3i32);
    
    assert(sum_of_two_enumerated_types(first, second));
}

}
