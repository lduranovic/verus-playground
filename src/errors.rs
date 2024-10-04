use vstd::prelude::*;

verus! {

// This should not be failing, but it is. There seems to be a spurious
// recommendation failure.
// #[allow(dead_code)]
// #[allow(unused_variables)]
// fn test(v: &Vec<i32>)
//     requires
//         v.len() == 1,
//     ensures
//         v@.subrange(0, 1).len() == v.len(),
// {
//     loop {
//         return;
//     }
// }

// The following fails. It seems that you cannot define a trait that takes a
// constant parameter.
// #[allow(dead_code)]
// pub trait Trait<const X: u64> {}

// proof fn subtake<T>(a: Seq<T>, b: Seq<T>, k: int, l: int)
//     requires
//         a.take(l) == b.take(l),
//         l <= a.len(),
//         l <= b.len(),
//         0 <= k <= l,
//     ensures
//         a.take(k) == b.take(k),
// {
//     assert(a.take(k).len() == b.take(k).len());
//     assert forall |i| 0 <= i < k implies a.take(k)[i] == b.take(k)[i] by {
//         assert(a.take(k)[i] == b.take(l)[i]);
//         assert(b.take(k)[i] == a.take(l)[i]);
//     }
//     // This punchline here is needed, but it should not be.
//     assert(a.take(k) == b.take(k));
// }

}
