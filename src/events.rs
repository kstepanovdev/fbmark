use std::sync::mpsc;

struct Event {
    input: Input(K),
}

struct Event {
    receiver: mscp::Receiver<Event<Key>>,
    input_handle: thread::JoinHandle<()>
}