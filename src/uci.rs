use std::sync::mpsc;
use std::sync::mpsc::{channel, Sender, TryRecvError};
use std::time::Duration;

use worker::Command;

pub fn uci_command(input: &str, sender: &Sender<Command>) -> &'static str {
    match input {
        "uci" => uci_result,
        "isready" => isready(sender),
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

fn isready(sender: &Sender<Command>) -> &'static str {
    let (response_sender, response_receiver) = channel();
    sender.send(Command::Ready(response_sender));
    response_receiver
        .recv_timeout(Duration::from_secs(60))
        .expect("Engine unresponsive");
    isready_result
}

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
    use std::thread;
    use uci::*;
    use worker::Command;

    #[test]
    fn uci_start_command() -> Result<(), String> {
        // Given
        let (sender, receiver) = mpsc::channel();

        // When
        let result = uci_command("uci", &sender);

        // Then
        assert_eq!(
            result,
            "id name woodpusher\nid author Sébastien Willmann\nuciok"
        );
        match receiver.try_recv() {
            Err(TryRecvError::Empty) => Ok(()),
            _ => Err(String::from("Expected empty receiver")),
        }
    }

    #[test]
    fn isready_command() {
        // Given
        let (sender, receiver) = mpsc::channel();
        let (test_sender, test_receiver) = mpsc::channel();
        let thread_sender = sender.clone();
        let thread_test_sender = test_sender.clone();

        // When
        let result = thread::spawn(move || {
            let r = uci_command("isready", &thread_sender);
            thread_test_sender.send(());
            r
        });

        // Then
        assert_eq!(test_receiver.try_recv(), Err(TryRecvError::Empty));

        // When
        match receiver.recv_timeout(Duration::from_secs(1)) {
            Ok(Command::Ready(response_sender)) => response_sender.send(()),
            x => panic!("Expected ready command, got {:?}", x),
        };

        // Then
        assert_eq!(test_receiver.recv_timeout(Duration::from_secs(1)), Ok(()));
        assert_eq!(result.join().unwrap(), "readyok");
    }

    #[test]
    fn position_startpos_command() -> Result<(), String> {
        // Given
        let (sender, receiver) = mpsc::channel();

        // When
        let result = uci_command("position startpos", &sender);

        // Then
        assert_eq!(result, "");
        match receiver.try_recv() {
            Ok(Command::StartPos) => Ok(()),
            _ => Err(String::from("Expected command startpos")),
        }
    }

    #[test]
    fn go_command() -> Result<(), String> {
        // Given
        let (sender, receiver) = mpsc::channel();

        // When
        let result = uci_command("go", &sender);

        // Then
        assert_eq!(result, "");
        match receiver.try_recv() {
            Ok(Command::Go) => Ok(()),
            _ => Err(String::from("Expected command go")),
        }
    }

    #[test]
    fn stop_command() -> Result<(), String> {
        // Given
        let (sender, receiver) = mpsc::channel();

        // When
        let result = uci_command("stop", &sender);

        // Then
        assert_eq!(result, "");
        match receiver.try_recv() {
            Ok(Command::Stop) => Ok(()),
            _ => Err(String::from("Expected command stop")),
        }
    }

    #[test]
    fn quit_command() -> Result<(), String> {
        // Given
        let (sender, receiver) = mpsc::channel();

        // When
        let result = uci_command("quit", &sender);

        // Then
        assert_eq!(result, "");
        match receiver.try_recv() {
            Ok(Command::Quit) => Ok(()),
            _ => Err(String::from("Expected command quit")),
        }
    }
}
