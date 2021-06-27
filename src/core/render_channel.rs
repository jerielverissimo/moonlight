use std::{
    mem,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Once,
    },
};

#[derive(Clone)]
pub struct NextFrame;

#[derive(Clone)]
pub struct RenderChannel {
    pub sender: Sender<NextFrame>,
    pub receiver: Arc<Receiver<NextFrame>>,
}

impl RenderChannel {
    pub fn new() -> Self {
        singleton()
    }

    pub fn sender(&self) -> Sender<NextFrame> {
        self.sender.clone()
    }

    pub fn next_frame(&self) {
        self.sender.send(NextFrame);
    }
}

fn singleton() -> RenderChannel {
    static mut RENDER_SCHEDULE: *const RenderChannel = 0 as *const RenderChannel;
    static ONCE: Once = Once::new();

    unsafe {
        ONCE.call_once(|| {
            let (tx, rx) = channel();
            let singleton = RenderChannel {
                sender: tx,
                receiver: Arc::new(rx),
            };

            RENDER_SCHEDULE = mem::transmute(Box::new(singleton));
        });

        (*RENDER_SCHEDULE).clone()
    }
}
