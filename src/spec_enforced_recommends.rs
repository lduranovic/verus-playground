use vstd::prelude::*;

verus! {

pub open spec(checked) fn remove_one(num: nat) -> int
    recommends
        num > 0
{
    num - 1
}

pub proof fn remove_one_sound(num: nat)
{
    let initial_value: nat = 2;
    let first_result: nat = remove_one(initial_value) as nat;
    assert(first_result == 1nat);
    let second_result: nat = remove_one(first_result) as nat;
    assert(second_result == 0nat);
    let final_result: nat = remove_one(second_result) as nat;
}

}
