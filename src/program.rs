use std::{
    cell::RefCell,
    io::Result,
    rc::Rc,
    sync::{
        mpsc::{channel, Sender},
        Mutex,
    },
    thread,
};

use addy::SIGWINCH;

use crate::{input::InputEvent, receive_inputs, renderer::Renderer, Channel};

static mut IS_RUNNING: Running = Running::Keep;
static mut RENDER_SENDER: Option<Mutex<Sender<ShouldRender>>> = None;

struct ShouldRender; // dummy msg

enum Running {
    Keep,
    Done,
}

pub fn quit() {
    unsafe {
        IS_RUNNING = Running::Done;
    }
}

pub(crate) fn schedule_render() {
    unsafe {
        let tx = RENDER_SENDER.as_ref().unwrap().lock().unwrap().clone();
        tx.send(ShouldRender).ok();
    }
}

pub type Sub<M, MSG> = Box<dyn Fn(&M) -> MSG + Send>;
pub type Cmd<MSG> = Box<dyn Fn() -> MSG + Send>;
pub type BatchCmd<MSG> = Vec<Cmd<MSG>>;

pub fn program<INI, M, U, MSG, V, I, S, C>(
    init: INI,
    mut update: U,
    view: V,
    input: I,
    subs: Vec<S>,
) -> Result<()>
where
    INI: Fn() -> (M, Option<Cmd<MSG>>) + Send + 'static,
    M: 'static + Send + Sync + Clone, // FIXME: remove clone necessity
    U: FnMut(MSG, &mut M) -> Vec<C> + 'static,
    MSG: 'static + Send + Sync,
    V: Fn(&M) -> String,
    I: Fn(InputEvent) -> Option<MSG> + Send + Sync + Copy + 'static,
    S: Fn(&M) -> MSG + Send + 'static,
    C: Fn() -> MSG + Send + 'static,
{
    let (tx, render_receiver) = channel();
    unsafe {
        RENDER_SENDER = Some(Mutex::new(tx));
    }

    let mut channel = Channel::new();
    let input_sender = channel.sender();

    // change terminal mode
    let mut renderer = Renderer::new();
    renderer.hide_cursor()?;

    // Initialize program
    let (mut model, cmd) = init();

    if let Some(cmd) = cmd {
        let mut cmd_sender = channel.sender();
        thread::spawn(move || {
            let msg = cmd();
            cmd_sender.send(msg);
        })
        .join()
        .expect("failed to join cmd thread");
    }

    let messages = Rc::new(RefCell::new(Vec::new()));

    // Render initial view
    let first_frame = view(&model);
    renderer.render(&first_frame)?;

    // input thread
    thread::spawn(move || {
        receive_inputs(input, input_sender);
    });

    let (w, h) = termion::terminal_size().expect("could not get terminal size");
    let (terminal_size_sender, terminal_size_receiver) = std::sync::mpsc::channel();
    let mut cross_channel_sender = channel.sender();
    thread::spawn(move || {
        for msg in terminal_size_receiver.iter() {
            cross_channel_sender.send(msg);
        }
    });

    let msg = input(InputEvent::WindowSize {
        width: w,
        height: h,
    });
    if let Some(msg) = msg {
        terminal_size_sender.send(msg);
        schedule_render();
    }

    addy::mediate(SIGWINCH)
        .register("terminal_size_change", move |_signal| {
            let (w, h) = termion::terminal_size().expect("could not get terminal size");
            let msg = input(InputEvent::WindowSize {
                width: w,
                height: h,
            });
            if let Some(msg) = msg {
                terminal_size_sender.send(msg);
                schedule_render();
            }
        })
        .expect("could not register input handler to terminal_size_change event")
        .enable()
        .expect("could not enable terminal_size_change event");

    // For each sub a new thread is created and executed infinitely, this makes sense?
    // TODO: maybe a model of pub/sub?
    // TODO: what happens when we have a lot subs?
    // TODO: how to unsubscribe?
    {
        for sub in subs {
            let model = model.clone(); // FIXME: find a better way to remove the clone!!!
            let mut sub_sender = channel.sender();
            thread::spawn(move || loop {
                // TODO: discovery a better way to handle subs,
                // and add a way to unsubscribe
                sub_sender.send(sub(&model));
            });
        }
    }

    // main loop, update states when MSG is received,
    // draw when ShouldRender is received
    for _ in render_receiver.iter() {
        let mut borrowed = messages.borrow_mut();
        borrowed.extend(channel.rx.try_iter());
        for msg in borrowed.drain(..) {
            for cmd in update(msg, &mut model).drain(..) {
                let mut cmd_sender = channel.sender();
                //thread::spawn(move || {
                let msg = cmd(); // TODO: process command concurrently, maybe async?
                cmd_sender.send(msg);
                //});
            }
        }

        let next_frame = view(&model);
        renderer.render(&next_frame)?;

        unsafe {
            if let Running::Done = IS_RUNNING {
                break;
            }
        }
    }

    renderer.show_cursor()?;
    renderer.restore_terminal()?;
    Ok(())
}
