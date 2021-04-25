use lunatic::{channel::unbounded, Process};

const COUNT: usize = 100_000;

fn main() {
    eprintln!("Spawning {} processes that add 1+2+3", COUNT);
    let (tx_response, rx_response) = unbounded::<u32>();

    let summer = Process::spawn_with(rx_response, |rx| {
        let mut sum = 0u128;
        for _ in 0..COUNT {
            sum += rx.receive().unwrap() as u128;
        }
        eprintln!("Sum: {}", sum);
    });

    for _ in 0..COUNT {
        Process::spawn_with((vec![1, 2, 3], tx_response.clone()), |(numbers, tx)| {
            let sum = numbers.into_iter().sum();
            tx.send(sum).unwrap();
        })
        .detach();
    }

    summer.join().unwrap();
}
