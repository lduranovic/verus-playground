#![allow(unused_imports)]
#![allow(dead_code)]

use builtin::*;
use builtin_macros::*;
use vstd::prelude::*;

verus! {

spec fn is_even(i: int) -> bool {
    i % 2 == 0
}

proof fn test_use_forall(s: Seq<int>)
    requires
        5 <= s.len(),
        forall |i: int| #![auto] 0 <= i < s.len() ==> is_even(s[i]),
{
    assert(s[3] % 2 == 0);
}

}
