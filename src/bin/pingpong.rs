use lunatic::{Process, channel::{Receiver, Sender, bounded}};


#[derive(serde::Serialize, serde::Deserialize)]
struct EchoRequest {
    message: String,
    chan: Sender<String>,
}

type EchoChan = Receiver<EchoRequest>;

struct EchoServer {
    chan: Sender<EchoRequest>,
}

impl EchoServer {

    fn run(chan: EchoChan) {
        while let Ok(req) = chan.receive() {
            req.chan.send(req.message).unwrap();
        }
    }

    fn spawn() -> Self {
        let (tx, rx) = lunatic::channel::bounded(1);
        Process::spawn_with(rx, Self::run).detach();
        Self{
            chan: tx,
        }
    }

    fn echo(&self, message: String) -> String {
        let (tx, rx) = bounded(1);
        self.chan.send(EchoRequest{
            message,
            chan: tx,
        }).unwrap();
        rx.receive().unwrap()
    }
}

fn main() {
    let count = 1_000_000;
    eprintln!("Sending and receiving {} echo messages", count);
    let echo = EchoServer::spawn();

    for i in 0..count {
        let msg = format!("hello {}", i);
        let res = echo.echo(msg.clone());
        assert_eq!(msg, res);
    }
}
