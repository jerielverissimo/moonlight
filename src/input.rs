use termion::event::Key;
use termion::input::TermRead;

use std::io::stdin;

pub(crate) fn receive_inputs<MSG, I>(input: I, mut input_sender: crate::ChannelSender<MSG>)
where
    I: Fn(Key) -> Option<MSG> + Send + 'static,
{
    let stdin = stdin();
    for c in stdin.keys() {
        match c.unwrap() {
            event => {
                if let Some(msg) = input(event) {
                    input_sender.send(msg);
                }
            }
        }
    }
}
