use crate::utils::error::{ProxyError, ProxyResult};
use std::sync::mpsc;
use std::time::Duration;

pub trait Delay {
    fn reset(&mut self);
    fn after(&mut self) -> ProxyResult<mpsc::Receiver<()>>;
    fn sleep(&mut self) -> ProxyResult<tokio::task::JoinHandle<()>>;
    fn sleep_with_cancel<F>(&mut self, canceled: F) -> ProxyResult<()>
    where
        F: Fn() -> bool + Send + Sync + 'static;
}

pub struct DelayExp2 {
    min: u32,
    max: u32,
    value: u32,
    unit: Duration,
}

impl DelayExp2 {
    pub fn new(min: u32, max: u32, unit: Duration) -> Self {
        DelayExp2 {
            min,
            max,
            value: 0,
            unit,
        }
    }

    fn next_value(&mut self) -> ProxyResult<u32> {
        self.value = self.value * 2;
        self.value = self.value.min(self.max).max(self.min);
        Ok(self.value)
    }
}

impl Delay for DelayExp2 {
    fn reset(&mut self) {
        self.value = 0;
    }

    fn after(&mut self) -> ProxyResult<mpsc::Receiver<()>> {
        let total = self.next_value().map_err(|e| ProxyError::NextDelay)?;
        let (sender, receiver) = mpsc::channel();
        let unit = self.unit;
        tokio::spawn(async move {
            tokio::time::sleep(unit * total).await;
            let _ = sender.send(());
        });
        Ok(receiver)
    }

    fn sleep(&mut self) -> ProxyResult<tokio::task::JoinHandle<()>> {
        let total = self.next_value().map_err(|e| ProxyError::NextDelay)?;
        let sleep_duration = self.unit * total;
        Ok(tokio::spawn(async move {
            tokio::time::sleep(sleep_duration).await;
        }))
    }

    fn sleep_with_cancel<F>(&mut self, canceled: F) -> ProxyResult<()>
    where
        F: Fn() -> bool + Send + Sync + 'static,
    {
        let total = self.next_value().map_err(|e| ProxyError::NextDelay)?;
        let unit = self.unit;
        tokio::spawn(async move {
            for _ in 0..total {
                if canceled() {
                    break;
                }
                tokio::time::sleep(unit).await;
            }
        });
        Ok(())
    }
}

mod tests {}
