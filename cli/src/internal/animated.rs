use std::borrow::Cow;
use std::future::Future;
use std::pin::Pin;
use tokio::time::Duration;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

pub struct Process<F, S> {
    pub func: F,
    pub on_start: S,
    pub on_end: S,
}
pub struct Animated {
    multi: MultiProgress,
    processes: Vec<Process<Pin<Box<dyn Future<Output = ()> + Send>>, Cow<'static, str>>>,
}

impl Animated {
    pub fn builder() -> Self {
        Self { multi: MultiProgress::new(), processes: vec!() }
    }

    pub fn add(mut self, process: Process<impl Future<Output = ()> + Send + 'static, impl Into<Cow<'static, str>>>) -> Self {
        self.processes.push(Process {
            func: Box::pin(process.func),
            on_start: process.on_start.into(),
            on_end: process.on_end.into(),
        });
        self
    }

    pub async fn invoke(self) {
        let mut h = vec!();
        for p in self.processes {
            let bar = self.multi.add(ProgressBar::new_spinner());
            bar.enable_steady_tick(Duration::from_millis(100));
            bar.set_style(ProgressStyle::with_template("{spinner:.magenta} [{elapsed_precise}] {wide_msg:.magenta}").unwrap());

            h.push(tokio::spawn(async move {
                bar.set_message(p.on_start);
                p.func.await;

                bar.set_style(ProgressStyle::with_template("{prefix:.bold.dim} {msg:.green} in {elapsed:.cyan}").unwrap());
                bar.set_prefix("✅");
                bar.finish_with_message(p.on_end);
                ()
            }))
        }

        for handle in h {
            handle.await.unwrap();
        }
    }

}