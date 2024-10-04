use vstd::prelude::*;

verus! {

// This should not be failing, but it is. There seems to be a spurious
// recommendation failure.
#[allow(dead_code)]
#[allow(unused_variables)]
fn test(v: &Vec<i32>)
    requires
        v.len() == 1,
    ensures
        v@.subrange(0, 1).len() == v.len(),
{
    loop {
        return;
    }
}

// The following fails. It seems that you cannot define a trait that takes a
// constant parameter.
#[allow(dead_code)]
pub trait Trait<const X: u64> {}

}
