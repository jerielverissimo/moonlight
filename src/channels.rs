use std::sync::mpsc::{channel, Receiver, Sender};

use crate::schedule_render;

pub struct ChannelSender<MSG> {
    pub tx: Sender<MSG>,
}

impl<MSG> ChannelSender<MSG> {
    pub fn send(&mut self, msg: MSG) {
        self.tx.send(msg).ok();
        schedule_render();
    }
}

pub struct Channel<MSG> {
    pub tx: Sender<MSG>,
    pub rx: Receiver<MSG>,
}

impl<MSG> Channel<MSG> {
    pub(crate) fn new() -> Self {
        let (tx, rx) = channel();
        return Channel { tx, rx };
    }

    pub fn sender(&mut self) -> ChannelSender<MSG> {
        return ChannelSender {
            tx: self.tx.clone(),
        };
    }
}
