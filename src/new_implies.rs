//! We have an executable function, which looks through a vector, and finds the
//! second smallest element.
//!
//! We also have a specification function, which works with a sequence right
//! now, looks through, and returns an element if there is one.

use vstd::prelude::*;

verus! {

spec fn divisible_by_twenty_four(n: nat) -> bool {
    n % 24 == 0
}

proof fn show_something(n: nat)
    requires
        n % 24 == 0
    ensures
        n % 2 == 0
{
    assert forall |k: nat| divisible_by_twenty_four(k) implies k % 2 == 0 by {
    }
}

}
