#![allow(dead_code)]

use vstd::prelude::*;

verus! {

pub struct Execution<T> {
    pub nat_to_state: spec_fn(nat) -> T
}

impl<T> Execution<T> {
    pub open spec fn head(self) -> T {
        (self.nat_to_state)(0)
    }

    pub open spec fn head_next(self) -> T {
        (self.nat_to_state)(1)
    }

    pub open spec fn suffix(self, pos: nat) -> Self {
        Execution {
            nat_to_state: |i: nat| (self.nat_to_state)(i + pos),
        }
    }
}

pub type StatePred<T> = spec_fn(T) -> bool;

pub type ActionPred<T> = spec_fn(T, T) -> bool;

#[verifier(reject_recursive_types(T))]
pub struct TempPred<T> {
    pub pred: spec_fn(Execution<T>) -> bool,
}

impl<T> TempPred<T> {
    pub open spec fn new(pred: spec_fn(Execution<T>) -> bool) -> Self {
        TempPred {
            pred: pred,
        }
    }

    pub open spec fn satisfied_by(self, execution: Execution<T>) -> bool {
        (self.pred)(execution)
    }

    /// We specify all infix operators for temporal logic as TempPred methods to aid readability

    /// `/\` for temporal predicates in TLA+ (i.e., `&&` in Verus).
    pub open spec fn and(self, other: Self) -> Self {
        TempPred::new(|ex: Execution<T>| self.satisfied_by(ex) && other.satisfied_by(ex))
    }

    /// `\/` for temporal predicates in TLA+ (i.e., `||` in Verus).
    pub open spec fn or(self, other: Self) -> Self {
        TempPred::new(|ex: Execution<T>| self.satisfied_by(ex) || other.satisfied_by(ex))
    }

    /// `=>` for temporal predicates in TLA+ (i.e., `==>` in Verus).
    pub open spec fn implies(self, other: Self) -> Self {
        TempPred::new(|ex: Execution<T>| self.satisfied_by(ex) ==> other.satisfied_by(ex))
    }

    /// `<=>` for temporal predicates in TLA+ (i.e., `<==>` in Verus).
    pub open spec fn equals(self, other: Self) -> Self {
        TempPred::new(|ex: Execution<T>| self.satisfied_by(ex) <==> other.satisfied_by(ex))
    }

    /// `~>` for temporal predicates in TLA+.
    /// Returns a temporal predicate that is satisfied
    /// iff it is always the case that `self` getting satisfied implies `other` eventually getting satisfied.
    ///
    /// Defined in 3.2.3.
    pub open spec fn leads_to(self, other: Self) -> Self {
        always(self.implies(eventually(other)))
    }

    /// `|=` for temporal predicates in TLA+.
    pub open spec fn entails(self, other: Self) -> bool {
        valid(self.implies(other))
    }
}

}
