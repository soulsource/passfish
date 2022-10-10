use std::sync::mpsc::{channel,Sender,Receiver,SendError};
#[derive(Clone)]
pub(crate) struct EmittingSender<MT, E : Fn()> {
    sender : Sender<MT>,
    emit : E,
}

impl <MT, E : Fn()> EmittingSender<MT, E>{
    pub(crate) fn emitting_channel(emit : E) -> (Self, Receiver<MT>) {
        let (sender,r) = channel();
        (EmittingSender { sender, emit }, r)
    }
    pub(crate) fn send(&self, t: MT) -> Result<(), SendError<MT>> {
        //first send, then notify.
        self.sender.send(t).map(|_| (self.emit)())
    }
}
