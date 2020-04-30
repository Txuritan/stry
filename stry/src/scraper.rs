use crate::worker::{Message, WorkerState};

pub fn runner(state: WorkerState, msg: Message) {
    match &msg {
        Message::Download { site, .. } => {
            state.site.store(Some(*site));
        }
        Message::Update { site, .. } => {
            state.site.store(Some(*site));
        }
    }
}
