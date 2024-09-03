//! We have an executable function, which looks through a vector, and finds the
//! second smallest element.
//!
//! We also have a specification function, which works with a sequence right
//! now, looks through, and returns an element if there is one.

use vstd::prelude::*;

verus! {

pub open spec fn find_second_smallest_spec(
    seq: Seq<nat>,
    value: nat
) -> Option<nat>
{
    if seq.len() <= 1 {
        None
    } else {
        let proper_element: nat = choose |n: nat| seq.contains(n);
    }
}

pub fn find_second_smallest(nums: &Vec<u64>) -> Option<u64>
    ensures
        find_second_smallest(nums@) == 5
{
    if nums.len() <= 1 {
        // There is just one element in the vector, so "second smallest" makes
        // no sense here.
        None
    } else {
        // Start off with the first element.
        let mut current_candidate: u64 = nums[0];
        let mut index: usize = 1;

        // Keep iterating through the rest; if you find something better, make
        // a note of it.
        while index < nums.len() {
            let other_candidate: u64 = nums[index];
            if other_candidate < current_candidate {
                current_candidate = other_candidate;
            }
            index += 1;
        }

        // Return the best so far.
        Some(current_candidate)
    }
}

}
