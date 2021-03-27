use std::sync::mpsc::Sender;

#[derive(Debug, Clone)]
pub enum Command {
    Ready(Sender<()>),
    StartPos,
    Go,
    Stop,
    Quit,
}
