//! This file shows how one can iterate through a sequence.

use builtin::*;
use builtin_macros::*;
use vstd::prelude::*;

verus! {

/// Go through a sequence one by one, just like you would with a `while` loop.
pub open spec fn iterate(seq: Seq<int>, i: nat) -> bool 
    decreases seq.len() - i,
{
    decreases_when(i < seq.len());
    decreases_by(proof_iterate);
        
    if i == seq.len() {
        true
    } else {
        iterate(seq, i + 1)
    }
}

/// Prove termination for this function.
#[verifier(decreases_by)]
proof fn proof_iterate(seq: Seq<int>, i: nat)
{
    if i == seq.len() {
    } else {
        assert(seq.len() - i > seq.len() - (i + 1));
    }
}

/// Find a given element in a sequence.
pub open spec fn find_element_index<V>(seq: &Seq<V>, i: int, v: V) -> Option<int>
    decreases seq.len() - i,
{
    decreases_when(0 <= i < seq.len());
    decreases_by(proof_find_element_index::<V>);

    if i == seq.len() {
        None
    } else {
        let element = seq.index(i);
        if element == v {
            Some(i)
        } else {
            find_element_index(seq, i + 1, v)
        }
    }
}

/// Prove that finding an element in a given sequence results in an expected
/// thing.
#[verifier(decreases_by)]
proof fn proof_find_element_index<V>(seq: &Seq<V>, i: int, v: V)
{
    if i == seq.len() {
    } else {
        let element = seq.index(i);
        if element == v {
        } else {
            assert(seq.len() - i > seq.len() - (i + 1));
        }
    }
}

}
