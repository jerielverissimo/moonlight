use std::{cell::RefCell, rc::Rc, sync::mpsc::channel};

use crate::render;

static mut IS_RUNNING: Running = Running::Keep;

struct ShouldRender;

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

pub fn program<M, U, MSG, V, I>(mut model: M, mut update: U, view: V, input: I)
where
    U: FnMut(MSG, &mut M),
    V: Fn(&M) -> String,
    I: Fn(&str) -> Option<MSG>,
{
    let (tx, rx) = channel();
    let (input_tx, input_rx) = channel();

    let messages = Rc::new(RefCell::new(Vec::new()));

    if let Some(msg) = input("") {
        input_tx.send(msg).ok();
        tx.send(ShouldRender).ok();
    }

    let mut callback = move || {
        let mut borrowed = messages.borrow_mut();
        borrowed.extend(input_rx.try_iter());
        for msg in borrowed.drain(..) {
            update(msg, &mut model);
        }

        let next_frame = view(&model);
        render(&next_frame);
    };

    for _ in rx.iter() {
        callback();

        unsafe {
            if let Running::Done = IS_RUNNING {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_call_update_fun() {
        // arrange
        let mut spy = Vec::new();

        struct Dummy(&'static str);

        enum DummyMsg {
            Update,
        }

        let update = |_msg: DummyMsg, m: &mut Dummy| {
            m.0 = "updated";
            spy.push(Dummy(m.0));
            super::quit();
        };

        let model = Dummy("dummy");

        let view = |m: &Dummy| m.0.to_string();

        let input = |_io: &str| Some(DummyMsg::Update);

        // act
        program(model, update, view, input);

        // assert
        assert_eq!(spy[0].0, "updated");
    }
}
