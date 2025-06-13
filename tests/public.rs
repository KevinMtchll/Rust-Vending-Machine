use std::{collections::HashMap, sync::Arc};

use vending_machine::{Fsm, VendingAction};

fn mk_hm(version: i32) -> HashMap<Arc<str>, u32> {
    let mut hm = HashMap::new();
    if version == 0 {
        hm.insert("pretzels".into(), 15);
        hm.insert("chips".into(), 10);
        hm.insert("popcorn".into(), 10);
    } else if version == 1 {
        hm.insert("pretzels".into(), 12);
        hm.insert("chips".into(), 7);
        hm.insert("popcorn".into(), 6);
    } else if version == 2 {
        hm.insert("candy".into(), 5);
        hm.insert("chips".into(), 10);
        hm.insert("popcorn".into(), 25);
    }
    hm
}

#[test]
fn public_test_accept1() {
    let hm = mk_hm(0);
    let fsm = Fsm::create(&hm);
    let mut actions1 = Vec::new();
    actions1.push(VendingAction::Insert(25));
    actions1.push(VendingAction::Vend("pretzels".into()));

    let mut actions2 = Vec::new();
    actions2.push(VendingAction::Insert(10));
    actions2.push(VendingAction::Vend("chips".into()));

    let mut actions3 = Vec::new();
    actions3.push(VendingAction::Insert(5));
    actions3.push(VendingAction::Insert(5));
    actions3.push(VendingAction::Vend("popcorn".into()));

    assert!(fsm.eval(&actions1));
    assert!(fsm.eval(&actions2));
    assert!(fsm.eval(&actions3));
}

#[test]
fn public_test_accept2() {
    let hm = mk_hm(0);
    let fsm = Fsm::create(&hm);
    let mut actions1 = Vec::new();
    actions1.push(VendingAction::Insert(5));
    actions1.push(VendingAction::Insert(5));
    actions1.push(VendingAction::Vend("pretzels".into()));

    let mut actions2 = Vec::new();
    actions2.push(VendingAction::Insert(5));
    actions2.push(VendingAction::Vend("chips".into()));

    let mut actions3 = Vec::new();
    actions3.push(VendingAction::Vend("popcorn".into()));

    assert!(!fsm.eval(&actions1));
    assert!(!fsm.eval(&actions2));
    assert!(!fsm.eval(&actions3));
}

#[test]
fn public_test_accept3() {
    let hm = mk_hm(1);
    let fsm = Fsm::create(&hm);
    let mut actions1 = Vec::new();
    actions1.push(VendingAction::Insert(25));
    actions1.push(VendingAction::Vend("pretzels".into()));

    let mut actions2 = Vec::new();
    actions2.push(VendingAction::Insert(10));
    actions2.push(VendingAction::Vend("chips".into()));

    let mut actions3 = Vec::new();
    actions3.push(VendingAction::Insert(5));
    actions3.push(VendingAction::Insert(5));
    actions3.push(VendingAction::Vend("popcorn".into()));
    assert!(fsm.eval(&actions1));
    assert!(fsm.eval(&actions2));
    assert!(fsm.eval(&actions3));
}

#[test]
fn public_test_accept4() {
    let hm = mk_hm(1);
    let fsm = Fsm::create(&hm);
    let mut actions1 = Vec::new();
    actions1.push(VendingAction::Insert(5));
    actions1.push(VendingAction::Insert(5));
    actions1.push(VendingAction::Vend("pretzels".into()));

    let mut actions2 = Vec::new();
    actions2.push(VendingAction::Insert(5));
    actions2.push(VendingAction::Vend("chips".into()));

    let mut actions3 = Vec::new();
    actions3.push(VendingAction::Vend("popcorn".into()));
    assert!(!fsm.eval(&actions1));
    assert!(!fsm.eval(&actions2));
    assert!(!fsm.eval(&actions3));
}

//--------------------------------------------------------

#[test]
fn public_test_accept5() {
    let hm = mk_hm(2);
    let fsm = Fsm::create(&hm);

    let mut actions1 = Vec::new();
    actions1.push(VendingAction::Insert(25));
    actions1.push(VendingAction::Vend("candy".into()));
    actions1.push(VendingAction::Vend("candy".into()));

    let mut actions2 = Vec::new();
    actions2.push(VendingAction::Insert(25));
    actions2.push(VendingAction::Vend("chips".into()));
    actions2.push(VendingAction::Vend("candy".into()));

    let mut actions3 = Vec::new();
    actions3.push(VendingAction::Insert(5));
    actions3.push(VendingAction::Insert(5));
    actions3.push(VendingAction::Vend("candy".into()));
    actions3.push(VendingAction::Vend("candy".into()));

    assert!(fsm.eval(&actions1));
    assert!(fsm.eval(&actions2));
    assert!(fsm.eval(&actions3));
}

#[test]
fn public_test_accept6() {
    let hm = mk_hm(2);
    let fsm = Fsm::create(&hm);

    let mut actions1 = Vec::new();
    actions1.push(VendingAction::Insert(10));
    actions1.push(VendingAction::Vend("candy".into()));
    actions1.push(VendingAction::Vend("chips".into()));

    let mut actions2 = Vec::new();
    actions2.push(VendingAction::Insert(10));
    actions2.push(VendingAction::Insert(5));
    actions2.push(VendingAction::Vend("chips".into()));
    actions2.push(VendingAction::Vend("candy".into()));

    let mut actions3 = Vec::new();
    actions3.push(VendingAction::Insert(5));
    actions3.push(VendingAction::Vend("candy".into()));
    actions3.push(VendingAction::Vend("chips".into()));

    assert!(!fsm.eval(&actions1));
    assert!(fsm.eval(&actions2));
    assert!(!fsm.eval(&actions3));
}
