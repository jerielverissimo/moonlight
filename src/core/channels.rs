use std::sync::mpsc::{channel, Receiver, Sender};

use super::render_channel::RenderChannel;

pub struct ChannelSender<MSG> {
    pub tx: Sender<MSG>,
}

impl<MSG> ChannelSender<MSG> {
    pub fn send(&mut self, msg: MSG) {
        self.tx.send(msg).ok();
        RenderChannel::new().next_frame();
    }
}

pub struct Channel<MSG> {
    pub tx: Sender<MSG>,
    pub rx: Receiver<MSG>,
}

impl<MSG> Channel<MSG> {
    pub(crate) fn new() -> Self {
        let (tx, rx) = channel();
        Channel { tx, rx }
    }

    pub fn sender(&mut self) -> ChannelSender<MSG> {
        ChannelSender {
            tx: self.tx.clone(),
        }
    }
}
