use crate::{log_expect::LogExpect, COUNT, SUCCESSES};
use crossterm::{execute, terminal};
use log::info;
use std::io::stdout;
use std::sync::{atomic::Ordering, mpsc::Sender};

pub struct JobManager {
    tx: Option<Sender<()>>,
    count: usize,
}

impl JobManager {
    pub fn init(&mut self, tx: Sender<()>) {
        self.tx = Some(tx);
        execute!(stdout(), terminal::SetTitle("[webb] Initializing"))
            .log_expect("Failed to set terminal title.");
    }

    pub fn new() -> Self {
        JobManager { tx: None, count: 0 }
    }

    pub fn allocate(&mut self) {
        self.count += 1;
        execute!(
            stdout(),
            terminal::SetTitle(&format!(
                "[webb] Scraping: {} task(s) running. [{}/{}]",
                self.count,
                SUCCESSES.load(Ordering::Relaxed),
                COUNT.load(Ordering::Relaxed)
            ))
        )
        .log_expect("Failed to set terminal title.");
    }

    pub fn deallocate(&mut self) {
        self.count -= 1;
        execute!(
            stdout(),
            terminal::SetTitle(&format!(
                "[webb] Scraping: {} task(s) running. [{}/{}]",
                self.count,
                SUCCESSES.load(Ordering::Relaxed),
                COUNT.load(Ordering::Relaxed)
            ))
        )
        .log_expect("Failed to set terminal title.");

        if self.count == 0 {
            info!("Job done. Sending termination signal.");
            self.tx
                .as_ref()
                .log_expect("Failed to get termination signal sender.")
                .send(())
                .log_expect("Failed to send termination signal.");
        }
    }
}
