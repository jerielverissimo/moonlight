use std::{cell::RefCell, io::Result, rc::Rc, sync::mpsc::Sender, thread};

use addy::Signal::SIGWINCH;

use crate::{
    input::{receive_inputs, InputEvent},
    renderer::{exit_fullscreen, fullscreen, Renderer},
    store::{Middleware, Subscription},
    Channel,
};

use super::{heartbeat::Heartbeat, render_channel::RenderChannel, store::Store};

pub type Initialize<Model, Message> = fn() -> (Model, Option<Cmd<Message>>);

pub type Cmd<Message> = Box<dyn Fn() -> Message + Send + Sync>;
pub type BatchCmd<Message> = Vec<Cmd<Message>>;

pub struct Runtime<Model: Sync, Message, Input, View, Reducer> {
    heartbeat: Heartbeat,
    store: Store<Model, Message, Reducer>,
    ignition: Option<Cmd<Message>>, // first command to execute
    input: Input,
    view: View,
}

impl<Model, Message, Input, View, Reducer, Command> Runtime<Model, Message, Input, View, Reducer>
where
    Model: Clone + Send + Sync + 'static,
    Message: 'static + Send + Sync + Clone,
    Input: Fn(InputEvent) -> Option<Message> + Send + Sync + Copy + 'static,
    View: 'static + Fn(&Model) -> String + Send,
    Reducer: Fn(Model, Message) -> (Model, Vec<Command>),
    Command: Fn() -> Message + Send + 'static,
{
    pub fn new(
        reducer: Reducer,
        initial: Initialize<Model, Message>,
        input: Input,
        view: View,
    ) -> Self {
        // Initialize program
        let (model, cmd) = initial();

        Self {
            heartbeat: Heartbeat::new(),
            store: Store::new(reducer, model),
            ignition: cmd,
            input,
            view,
        }
    }

    pub fn with_fullscreen(self) -> Self {
        fullscreen();
        self
    }

    /// Will execute before reducer
    pub fn with_middleware(mut self, middleware: Middleware<Model, Message, Reducer>) -> Self {
        self.store.middleware(middleware); // will excute before reducer
        self
    }

    /// Will execute every time
    pub fn with_subscription(mut self, subscription: Subscription<Model, Message>) -> Self {
        self.store.subscribe(subscription);
        self
    }

    pub fn run(mut self) -> Result<()> {
        let render_channel = RenderChannel::new();
        let render_receiver = render_channel.receiver.clone();

        let mut channel = Channel::new();
        let input_sender = channel.sender();

        // change terminal mode
        let renderer = Rc::new(RefCell::new(Renderer::new()));
        renderer.borrow_mut().hide_cursor()?;

        // execute fist command
        if let Some(cmd) = self.ignition {
            execute(&mut channel, cmd);
        }

        let messages = Rc::new(RefCell::new(Vec::new()));

        // Render initial view
        let first_frame = (self.view)(&self.store.model());
        renderer.borrow_mut().render(&first_frame)?;

        // input thread
        thread::spawn(move || {
            receive_inputs(self.input, input_sender);
        });

        let (w, h, terminal_size_sender) = send_terminal_resize_message(&mut channel);

        let msg = (self.input)(InputEvent::WindowSize {
            width: w,
            height: h,
        });

        if let Some(msg) = msg {
            terminal_size_sender.send(msg);
            render_channel.next_frame();
        }

        // watch terminal size changes
        addy::mediate(SIGWINCH)
            .register("terminal_size_change", move |_signal| {
                let (w, h) = termion::terminal_size().expect("could not get terminal size");
                let msg = &(self.input)(InputEvent::WindowSize {
                    width: w,
                    height: h,
                });
                if let Some(msg) = msg {
                    terminal_size_sender.send(msg.clone());
                    let render_channel = RenderChannel::new();
                    render_channel.next_frame();
                }
            })
            .expect("could not register input handler to terminal_size_change event")
            .enable()
            .expect("could not enable terminal_size_change event");

        {
            // render callback
            let renderer = renderer.clone();
            self.store.react(move |model| {
                let next_frame = (self.view)(model);
                renderer.borrow_mut().render(&next_frame);
            });
        }

        self.store = self.store.dispatch_subscriptions(&mut channel);

        // main loop, update states when MSG is received,
        // draw when ShouldRender is received
        for _ in render_receiver.iter() {
            let mut borrowed = messages.borrow_mut();
            borrowed.extend(channel.rx.try_iter());
            for msg in borrowed.drain(..) {
                let cmd_sender = channel.sender();
                // dispatch reducer and execute render callback
                self.store.dispatch(msg, cmd_sender);
            }

            if self.heartbeat.is_dead() {
                exit_fullscreen();
                break;
            }
        }
        renderer.borrow_mut().show_cursor()?;
        renderer.borrow_mut().restore_terminal()?;
        Ok(())
    }
}

fn send_terminal_resize_message<Message: Send + Sync + 'static>(
    channel: &mut Channel<Message>,
) -> (u16, u16, Sender<Message>) {
    let (w, h) = termion::terminal_size().expect("could not get terminal size");
    let (terminal_size_sender, terminal_size_receiver) = std::sync::mpsc::channel();
    let mut cross_channel_sender = channel.sender();
    thread::spawn(move || {
        for msg in terminal_size_receiver.iter() {
            cross_channel_sender.send(msg);
        }
    });
    (w, h, terminal_size_sender)
}

fn execute<Message: Send + Sync + 'static>(
    channel: &mut Channel<Message>,
    cmd: Box<dyn Fn() -> Message + Send + Sync>,
) {
    let mut cmd_sender = channel.sender();
    thread::spawn(move || {
        let msg = cmd();
        cmd_sender.send(msg);
    })
    .join()
    .expect("failed to join cmd thread");
}
