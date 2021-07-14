use std::{thread};

use generational_arena::{Arena};

use crate::{BatchCmd, Channel, ChannelSender};

pub type Middleware<Model, Message, Reducer> =
    fn(&mut Store<Model, Message, Reducer>, Message) -> Option<Message>;
pub type Reaction<Model> = dyn Fn(&Model);
pub type Subscription<Model, Message> = fn(&Model) -> Message;
pub type Reducer<Model, Message> = fn(Model, Message) -> (Model, BatchCmd<Message>);

pub struct Store<Model: Sync, Message, Reducer> {
    reducer: Reducer,
    model: Model,
    middleware: Vec<Middleware<Model, Message, Reducer>>,
    reactions: Arena<Box<Reaction<Model>>>,
    subscriptions: Vec<Subscription<Model, Message>>,
}

impl<Model: Clone + Send + Sync + 'static, Message: Send + 'static, Reducer, Command>
    Store<Model, Message, Reducer>
where
    Reducer: Fn(Model, Message) -> (Model, Vec<Command>),
    Command: Fn() -> Message + Send + 'static,
{
    pub fn new(reducer: Reducer, initial: Model) -> Self {
        Self {
            reducer,
            model: initial,
            middleware: vec![],
            reactions: Arena::new(),
            subscriptions: Vec::new(),
        }
    }

    pub fn model(&self) -> Model {
        self.model.clone()
    }

    pub fn dispatch(&mut self, message: Message, cmd_sender: ChannelSender<Message>) {
        if self.middleware.is_empty() {
            self.dispatch_reducer(message, cmd_sender);
        } else {
            self.dispatch_middleware(0, message, cmd_sender);
        }
    }

    pub fn react<React: 'static + Fn(&Model)>(&mut self, callback: React) {
        self.reactions.insert(Box::new(callback));
    }

    pub fn subscribe(&mut self, callback: Subscription<Model, Message>) {
        self.subscriptions.push(callback);
    }

    pub fn middleware(&mut self, callback: Middleware<Model, Message, Reducer>) {
        self.middleware.push(callback);
    }

    pub fn dispatch_subscriptions(self, channel: &mut Channel<Message>) -> Self {
        for subscription in self.subscriptions.clone() {
            let model = self.model(); // needs clone :(
            let mut subscription_sender = channel.sender();
            thread::spawn(move || loop {
                let message = subscription(&model);
                subscription_sender.send(message);
            });
        }
        self
    }

    fn dispatch_middleware(
        &mut self,
        index: usize,
        message: Message,
        cmd_sender: ChannelSender<Message>,
    ) {
        if index == self.middleware.len() {
            self.dispatch_reducer(message, cmd_sender);
            return;
        }

        let next = self.middleware[index](self, message);

        if next.is_none() {
            return;
        }

        self.dispatch_middleware(index + 1, next.unwrap(), cmd_sender);
    }

    fn dispatch_reducer(&mut self, message: Message, mut cmd_sender: ChannelSender<Message>) {
        let (model, mut cmds) = (&self.reducer)(self.model(), message);
        self.model = model;

        for cmd in cmds.drain(..) {
            let msg = cmd();
            cmd_sender.send(msg);
        }

        self.dispatch_reactions(&self.model);
    }

    fn dispatch_reactions(&self, model: &Model) {
        for (_, reaction) in &self.reactions {
            reaction(model);
        }
    }
}
