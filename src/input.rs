pub(crate) fn receive_inputs<MSG, I>(input: I, mut input_sender: crate::ChannelSender<MSG>)
where
    I: Fn(&str) -> Option<MSG> + Send + 'static,
{
    if let Some(msg) = input("") {
        input_sender.send(msg);
    }
}
