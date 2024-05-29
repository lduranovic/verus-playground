#![allow(unused_imports)]
#![allow(dead_code)]

use builtin::*;
use builtin_macros::*;
use vstd::prelude::*;

verus! {

    /// The aim of this `spec` function is to test whether the short-circuit
    /// property makes sense, when broadly used with `spec` functions. The
    /// first condition in this function establishes that the sequence is not
    /// `None`, while the second condition is supposed to panic if that is not
    /// the case. Basically, I think this is fine.
    pub open spec fn work_on_sequence(
        s: Option<Seq<int>>,
        i: int
    ) -> bool {
        &&& s.is_some()
        &&& s.unwrap().contains(i)
    }

    fn test_work_on_sequence()
    {
        assert({
            let input_sequence = seq![1int, 2int, 3int];
            let input_integer = 1int;

            work_on_sequence(Some(input_sequence), input_integer)
        }) by {
            let input_sequence = seq![1int, 2int, 3int];
            let input_integer = 1int;

            assert(input_sequence.index(0) === input_integer);
        }
    }

}
