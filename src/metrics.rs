use indexmap::IndexMap;
use std::time::Duration;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Metric {
    pub name: String,
    pub duration: Duration,
}

#[derive(Default, Debug)]
pub enum Metrics {
    #[default]
    None,
    Some {
        ongoing: IndexMap<String, Instant>,
        completed: IndexMap<String, Duration>,
    },
}

impl Metrics {
    pub fn new() -> Self {
        Metrics::Some {
            ongoing: Default::default(),
            completed: Default::default(),
        }
    }
    pub fn begin(&mut self, name: &str) {
        match self {
            Metrics::None => todo!(),
            Metrics::Some { ongoing, .. } => {
                ongoing.insert(name.to_string(), Instant::now());
            }
        }
    }

    pub fn end(&mut self, name: &str) {
        match self {
            Metrics::None => todo!(),
            Metrics::Some { ongoing, completed } => {
                if let Some(start_time) = ongoing.shift_remove(name) {
                    let duration = start_time.elapsed();
                    completed.insert(name.to_string(), duration);
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn report(&self) -> String {
        match self {
            Metrics::None => "Metrics disabled, nothing to report".to_string(),
            Metrics::Some { completed, .. } => {
                format!(
                    "{} | total {:?}",
                    completed
                        .iter()
                        .map(|(name, duration)| format!("{} {:?}", name, duration))
                        .collect::<Vec<_>>()
                        .join(" | "),
                    &self.total()
                )
            }
        }
    }

    pub fn total(&self) -> Duration {
        match self {
            Metrics::None => Duration::ZERO,
            Metrics::Some { completed, .. } => completed.values().sum(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_tracking() {
        let mut metrics = Metrics::default();

        metrics.begin("z first");
        std::thread::sleep(Duration::from_millis(10));
        metrics.end("z first");

        metrics.begin("a second");
        std::thread::sleep(Duration::from_millis(20));
        metrics.end("a second");

        let report = metrics.report();
        println!("{}", report);
        assert!(report.contains("z first"));
        assert!(report.contains("a second"));
    }
}
