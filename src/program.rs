use std::sync::mpsc::channel;

pub fn program<M, U, MSG, V, I>(mut model: M, mut update: U, _view: V, input: I)
where
    U: FnMut(MSG, &mut M),
    V: Fn(&M) -> String,
    I: Fn(&str) -> Option<MSG>,
{
    let (tx, rx) = channel();

    let mut messages = Vec::new();

    if let Some(msg) = input("") {
        tx.send(msg).ok();
    }

    messages.extend(rx.try_iter());

    for msg in messages.drain(..) {
        update(msg, &mut model);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_call_update_fun() {
        // arrange
        let mut spy = Vec::new();

        struct Dummy(&'static str);

        enum DummyMsg {
            Update,
        }

        let update = |_msg: DummyMsg, m: &mut Dummy| {
            m.0 = "updated";
            spy.push(Dummy(m.0));
        };

        let model = Dummy("dummy");

        let view = |m: &Dummy| m.0.to_string();

        let input = |_io: &str| Some(DummyMsg::Update);

        // act
        program(model, update, view, input);

        // assert
        assert_eq!(spy[0].0, "updated");
    }
}
