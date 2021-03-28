use std::{
    cell::RefCell,
    io::stdin,
    rc::Rc,
    sync::{
        mpsc::{channel, Sender},
        Mutex,
    },
    thread,
};

use crate::{receive_inputs, render, Channel};

static mut IS_RUNNING: Running = Running::Keep;
static mut RENDER_SENDER: Option<Mutex<Sender<ShouldRender>>> = None;

struct ShouldRender; // dummy msg

enum Running {
    Keep,
    Done,
}

pub fn quit() {
    unsafe {
        println!("exiting");
        IS_RUNNING = Running::Done;
    }
}

pub(crate) fn schedule_render() {
    unsafe {
        let tx = RENDER_SENDER.as_ref().unwrap().lock().unwrap().clone();
        tx.send(ShouldRender).ok();
    }
}

pub fn program<M, U, MSG, V, I>(mut model: M, mut update: U, view: V, input: I)
where
    U: FnMut(MSG, &mut M),
    MSG: 'static + Send,
    V: Fn(&M) -> String,
    I: Fn(&str) -> Option<MSG> + Send + 'static,
{
    let (tx, render_receiver) = channel();
    unsafe {
        RENDER_SENDER = Some(Mutex::new(tx));
    }

    let mut channel = Channel::new();
    let input_sender = channel.sender();

    let messages = Rc::new(RefCell::new(Vec::new()));

    // input thread
    thread::spawn(move || {
        receive_inputs(input, input_sender);
    });

    let mut callback = move || {
        let mut borrowed = messages.borrow_mut();
        borrowed.extend(channel.rx.try_iter());
        for msg in borrowed.drain(..) {
            update(msg, &mut model);
        }

        let next_frame = view(&model);
        render(&next_frame);
    };

    // main loop, update states when MSG is received,
    // draw when ShouldRender is received
    for _ in render_receiver.iter() {
        callback();

        unsafe {
            if let Running::Done = IS_RUNNING {
                break;
            }
        }
    }
}
