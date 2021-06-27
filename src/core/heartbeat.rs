use std::{
    mem,
    sync::{Arc, Mutex, Once},
};

pub(crate) enum Health {
    Alive,
    Dead,
}

#[derive(Clone)]
pub struct Heartbeat {
    is_alive: Arc<Mutex<Health>>,
}

impl Heartbeat {
    pub(crate) fn new() -> Self {
        singleton()
    }

    pub(crate) fn is_dead(&self) -> bool {
        let alive = &*self.is_alive.lock().unwrap();
        match alive {
            Health::Alive => false,
            Health::Dead => true,
        }
    }

    fn kill(&self) {
        let mut alive = self.is_alive.lock().unwrap();
        *alive = Health::Dead;
    }

    pub fn stop() {
        let heartbeat = Heartbeat::new();
        heartbeat.kill();
    }
}

fn singleton() -> Heartbeat {
    static mut HEARTBEAT: *const Heartbeat = 0 as *const Heartbeat;
    static ONCE: Once = Once::new();

    unsafe {
        ONCE.call_once(|| {
            let singleton = Heartbeat {
                is_alive: Arc::new(Mutex::new(Health::Alive)),
            };

            HEARTBEAT = mem::transmute(Box::new(singleton));
        });

        (*HEARTBEAT).clone()
    }
}
