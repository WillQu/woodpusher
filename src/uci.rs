use std::sync::mpsc;
use std::sync::mpsc::{Sender, TryRecvError};
use worker::Command;

pub fn uci(input: &str, sender: &Sender<Command>) -> &'static str {
    match input {
        "uci" => uci_result,
        "isready" => isready_result,
        "position startpos" => startpos(sender),
        "go" => go(sender),
        "stop" => stop(sender),
        "quit" => quit(sender),
        _ => "",
    }
}

const uci_result: &str = "id name woodpusher\n\
    id author Sébastien Willmann\n\
    uciok";

const isready_result: &str = "readyok";

fn startpos(sender: &Sender<Command>) -> &'static str {
    sender.send(Command::StartPos);
    ""
}

fn go(sender: &Sender<Command>) -> &'static str {
    sender.send(Command::Go);
    ""
}

fn stop(sender: &Sender<Command>) -> &'static str {
    sender.send(Command::Stop);
    ""
}

fn quit(sender: &Sender<Command>) -> &'static str {
    sender.send(Command::Quit);
    ""
}

#[cfg(test)]
mod tests {
    use uci::*;
    use worker::Command;

    #[test]
    fn uci_command() {
        // Given
        let (sender, receiver) = mpsc::channel();

        // When
        let result = uci("uci", &sender);

        // Then
        assert_eq!(
            result,
            "id name woodpusher\nid author Sébastien Willmann\nuciok"
        );
        assert_eq!(receiver.try_recv(), Err(TryRecvError::Empty));
    }

    #[test]
    fn isready_command() {
        // Given
        let (sender, receiver) = mpsc::channel();

        // When
        let result = uci("isready", &sender);

        // Then
        assert_eq!(result, "readyok");
        assert_eq!(receiver.try_recv(), Err(TryRecvError::Empty));
    }

    #[test]
    fn position_startpos_command() {
        // Given
        let (sender, receiver) = mpsc::channel();

        // When
        let result = uci("position startpos", &sender);

        // Then
        assert_eq!(result, "");
        assert_eq!(receiver.try_recv(), Ok(Command::StartPos));
    }

    #[test]
    fn go_command() {
        // Given
        let (sender, receiver) = mpsc::channel();

        // When
        let result = uci("go", &sender);

        // Then
        assert_eq!(result, "");
        assert_eq!(receiver.try_recv(), Ok(Command::Go));
    }

    #[test]
    fn stop_command() {
        // Given
        let (sender, receiver) = mpsc::channel();

        // When
        let result = uci("stop", &sender);

        // Then
        assert_eq!(result, "");
        assert_eq!(receiver.try_recv(), Ok(Command::Stop));
    }

    #[test]
    fn quit_command() {
        // Given
        let (sender, receiver) = mpsc::channel();

        // When
        let result = uci("quit", &sender);

        // Then
        assert_eq!(result, "");
        assert_eq!(receiver.try_recv(), Ok(Command::Quit));
    }
}
