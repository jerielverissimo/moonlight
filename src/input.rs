use termion::input::TermRead;

use std::io::stdin;

pub(crate) fn receive_inputs<MSG, I>(input: I, mut input_sender: crate::ChannelSender<MSG>)
where
    I: Fn(&str) -> Option<MSG> + Send + 'static,
{
    let stdin = stdin();
    for c in stdin.keys() {
        match c.unwrap() {
            _ => {
                if let Some(msg) = input("") {
                    input_sender.send(msg);
                }
            }
        }
    }
}
