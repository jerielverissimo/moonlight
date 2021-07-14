use std::{thread::sleep, time::Duration};

use crate::{BatchCmd, Cmd};

pub fn tick<MSG: 'static>(d: Duration, fun: impl Fn() -> MSG + Send + Sync + 'static) -> Cmd<MSG> {
    Box::new(move || {
        sleep(d);
        fun()
    })
}

pub fn map_batch<I: 'static, O>(cmds: BatchCmd<I>) -> Vec<impl Fn() -> O + Send + 'static>
where
    O: From<I>,
{
    cmds.into_iter().map(|f| move || O::from(f())).collect()
}

pub fn map<I: 'static + Sync, O>(cmd: Cmd<I>) -> impl Fn() -> O + Send + 'static
where
    O: From<I>,
{
    move || O::from(cmd())
}
