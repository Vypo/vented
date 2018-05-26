extern crate vented;

use std::thread;

use vented::error::Error as VentError;
use vented::{Receiver, Sender};

#[test]
fn duplicate_receiver() {
    let _r1 = Receiver::create("duplicate_receiver").unwrap();

    match Receiver::create("duplicate_receiver") {
        Ok(_) => panic!("create should have failed, but didn't"),
        Err(VentError::AlreadyExists(_)) => (),
        Err(_) => panic!("create should have failed with AlreadyExists"),
    }
}

#[test]
fn no_receiver() {
    match Sender::open("no_receiver") {
        Ok(_) => panic!("open should have failed, but didn't"),
        Err(VentError::NotFound) => (),
        Err(_) => panic!("open should have failed with NotFound"),
    }
}

#[test]
fn wait_empty() {
    let receiver = Receiver::create("wait_empty").unwrap();

    match receiver.try_wait() {
        Ok(_) => panic!("try_wait should have failed, but didn't"),
        Err(VentError::WouldBlock) => (),
        Err(_) => panic!("try_wait should have failed with WouldBlock"),
    }
}

#[test]
fn post_wait_once() {
    let receiver = Receiver::create("post_wait_once").unwrap();
    let sender = Sender::open("post_wait_once").unwrap();

    sender.try_post().unwrap();
    receiver.try_wait().unwrap();
}

#[test]
fn wait_post_once() {
    let receiver = Receiver::create("wait_post_once").unwrap();
    let sender = Sender::open("wait_post_once").unwrap();

    let handle = thread::spawn(move || {
        while let Err(VentError::WouldBlock) = receiver.try_wait() {
            thread::yield_now();
        }
    });

    sender.try_post().unwrap();

    handle.join().unwrap();
}
