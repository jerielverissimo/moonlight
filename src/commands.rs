use std::{thread::sleep, time::Duration};

use crate::{BatchCmd, Cmd};

pub fn tick<MSG: 'static>(d: Duration, fun: impl Fn() -> MSG + Send + 'static) -> Cmd<MSG> {
    Box::new(move || {
        sleep(d);
        return fun();
    })
}

pub fn map<I: 'static, O>(cmds: BatchCmd<I>) -> Vec<impl Fn() -> O + Send + 'static>
where
    O: From<I>,
{
    cmds.into_iter().map(|f| move || O::from(f())).collect()
}
