//! Experimenting with sequences and mapping them to something else.

#![allow(dead_code)]

use builtin::*;
use builtin_macros::*;
use vstd::{
    *,
    prelude::*,
    pervasive::*,
    set_lib::*,
    map_lib::*,
    relations::*,
    seq_lib::*
};

verus! {

pub type HostID = int;

pub enum Value {
    Zero,
    One
}

pub enum Status {
    Initial,
    Echo,
}

pub struct Host {
    pub id: HostID,
    pub status: Status,
}

pub enum Message {
    Initial(Value),
    Echo(Value),
    Ready(Value),
}

pub struct Packet {
    pub src: HostID,
    pub msg: Message,
    pub dst: HostID,
}

pub struct State {
    pub hosts: Set<HostID>,
    pub faulty: Set<HostID>,
    pub correct: Map<HostID, Host>,
    pub leader: nat,
}

pub open spec fn construct_packets_differently(
    hosts: Set<HostID>,
    src: HostID,
    msg: Message,
) -> Set<Packet>
    recommends
        hosts.finite()
{
    let host_sequence: Seq<HostID> = hosts.to_seq();
    let packet_sequence = host_sequence.map_values(|id: HostID| Packet {
        src,
        msg,
        dst: id,
    });
    packet_sequence.to_set()
}

pub proof fn packets_same_as_hosts(
    hosts: Set<HostID>,
    src: HostID,
    msg: Message,
    packets: Set<Packet>,
)
    requires
        hosts.finite(),
        packets =~= construct_packets_differently(
            hosts,
            src,
            msg
        )
    ensures
        packets.finite(),
        packets.len() == hosts.len()
{
    let host_sequence: Seq<HostID> = hosts.to_seq();
    assert(host_sequence.no_duplicates()) by {
        to_seq_no_duplicates(hosts, host_sequence);
    };
    assert(host_sequence.len() == hosts.len()) by {
        to_seq_same_length(hosts, host_sequence);
    };

    let packet_sequence = host_sequence.map_values(|id: HostID| Packet {
        src,
        msg,
        dst: id,
    });
    assert(packet_sequence.len() == host_sequence.len());
    assert(packet_sequence.no_duplicates());

    packet_sequence.unique_seq_to_set();
    let other_packets = packet_sequence.to_set();
    assert(other_packets.len() == packet_sequence.len());
    assert(other_packets =~= packets);
}

pub proof fn to_seq_preserves_elements<A>(set: Set<A>, seq: Seq<A>)
    requires
        set.finite(),
        seq =~= set.to_seq(),
    ensures
        forall |e: A| set.contains(e) <==> seq.contains(e)
    decreases
        set.len(),
        seq.len()
{
    if set.len() == 0 {
        assert(seq =~= Seq::<A>::empty());
    } else {
        let x = set.choose();
        assert(set.contains(x));
        assert(seq =~= Seq::<A>::empty().push(x) + set.remove(x).to_seq());
        let index: int = 0;
        assert(seq[index] == x);
        assert(seq.contains(x));

        let new_seq: Seq<A> = seq.remove(index);
        assert(new_seq.len() < seq.len());
        let new_set: Set<A> = set.remove(x);
        assert(new_set.len() < set.len());

        to_seq_preserves_elements(new_set, new_seq);
    }
}

pub proof fn to_seq_no_duplicates<A>(set: Set<A>, seq: Seq<A>)
    requires
        set.finite(),
        seq =~= set.to_seq()
    ensures
        seq.no_duplicates()
    decreases
        set.len(),
{
    if set.len() == 0 {
        to_seq_preserves_elements(set, seq);
        assert(seq.no_duplicates());
    } else {
        to_seq_preserves_elements(set, seq);
        assert(set.len() > 0);
        assert(seq.len() > 0) by {
            let x = set.choose();
            assert(seq.contains(x));
        };

        let x = set.choose();
        assert(seq =~= Seq::empty().push(x) + set.remove(x).to_seq());
        assert(seq.contains(x));
        assert(seq[0] == x);

        seq.remove_ensures(0);
        assert(set.remove(x).to_seq() =~= seq.remove(0));
        assert(seq.remove(0).contains(x) == false) by {
            to_seq_preserves_elements(set.remove(x), seq.remove(0));
        };
        assert(forall |i: int| 0 <= i < seq.len() && i != 0 ==> #[trigger] seq[i] != seq[0]);
        assert(seq.remove(0).no_duplicates() ==> seq.no_duplicates());

        to_seq_no_duplicates(set.remove(x), seq.remove(0));
    }
}

pub proof fn to_seq_to_set_same<A>(set: Set<A>, seq: Seq<A>)
    requires
        set.finite(),
        seq =~= set.to_seq()
    ensures
        seq.to_set() =~= set,
{
    to_seq_preserves_elements(set, seq);
    to_seq_no_duplicates(set, seq);
    seq.unique_seq_to_set();
}

pub proof fn to_seq_same_length<A>(set: Set<A>, seq: Seq<A>)
    requires
        set.finite(),
        seq =~= set.to_seq()
    ensures
        seq.len() == set.len()
    decreases
        set.len()
{
    if set.len() == 0 {
        assert(seq.len() == 0);
    } else {
        to_seq_preserves_elements(set, seq);
        to_seq_no_duplicates(set, seq);

        assert(set.len() > 0);
        let x = set.choose();
        assert(set.contains(x));
        assert(seq =~= Seq::empty().push(x) + set.remove(x).to_seq());
        assert(seq[0] == x);
        seq.remove_ensures(0);
        assert(seq.remove(0).contains(x) == false);
        assert(seq.remove(0).len() == seq.len() - 1);
        assert(set.remove(x).len() == set.len() - 1);
        assert(seq.remove(0) =~= set.remove(x).to_seq());

        to_seq_same_length(set.remove(x), seq.remove(0));
    }
}

}
