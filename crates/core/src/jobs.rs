//! CPU jobs and worker-to-app channels (`rayon` + `flume`, P0-07/P0-08).
//!
//! Rules enforced by this boundary:
//!
//! - Rayon runs pure, short-lived CPU work (validation, geometry math,
//!   export preprocessing). Workers never mutate editor state.
//! - Workers report back through typed [`JobChannel`]s. There is no global
//!   event bus and no `Arc<Mutex<Everything>>`: the application owns the
//!   single receiving end and drains it on its own tick.

use rayon::prelude::*;

/// Parallel sum of finite vertex positions checksum.
///
/// Pure function used by mesh validation and export preprocessing; `NaN`/`inf`
/// inputs are rejected with an error instead of silently poisoning the sum.
pub fn par_finite_sum(values: &[f32]) -> Result<f64, JobsError> {
    if values.iter().any(|v| !v.is_finite()) {
        return Err(JobsError::NonFiniteInput);
    }
    Ok(values.par_iter().map(|v| f64::from(*v)).sum())
}

/// Parallel per-item validation. The predicate must be pure and cheap; items
/// are processed in parallel but results keep input order.
pub fn par_validate<T, E>(
    items: &[T],
    check: impl Fn(&T) -> Result<(), E> + Sync + Send,
) -> Vec<Result<(), E>>
where
    E: Send,
    T: Sync,
{
    items.par_iter().map(check).collect()
}

/// Errors produced before or during job execution.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum JobsError {
    /// Input contained `NaN` or infinite values.
    #[error("job input contains non-finite values")]
    NonFiniteInput,
    /// The worker side was dropped before sending a result.
    #[error("job worker disconnected")]
    Disconnected,
}

/// Bounded typed channel from one worker (or worker pool) to the single
/// application owner. Dropping the sender disconnects the receiver.
#[derive(Debug)]
pub struct JobChannel<T> {
    sender: flume::Sender<T>,
    receiver: flume::Receiver<T>,
}

impl<T> JobChannel<T> {
    /// Creates a channel buffering up to `capacity` messages.
    pub fn bounded(capacity: usize) -> Self {
        let (sender, receiver) = flume::bounded(capacity);
        Self { sender, receiver }
    }

    /// Sends a result without blocking. Fails when full or disconnected.
    pub fn try_send(&self, value: T) -> Result<(), flume::TrySendError<T>> {
        self.sender.try_send(value)
    }

    /// Drains every buffered message without blocking.
    pub fn drain(&self) -> Vec<T> {
        self.receiver.try_iter().collect()
    }

    /// Clones the sending end for hand-off to a worker thread.
    pub fn sender(&self) -> flume::Sender<T> {
        self.sender.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parallel_sum_matches_sequential() {
        let values: Vec<f32> = (0..10_000).map(|i| (i as f32) * 0.25).collect();
        let expected: f64 = values.iter().map(|v| f64::from(*v)).sum();
        assert_eq!(par_finite_sum(&values).unwrap(), expected);
    }

    #[test]
    fn parallel_sum_rejects_non_finite() {
        assert_eq!(
            par_finite_sum(&[1.0, f32::NAN]),
            Err(JobsError::NonFiniteInput)
        );
        assert_eq!(
            par_finite_sum(&[f32::INFINITY]),
            Err(JobsError::NonFiniteInput)
        );
    }

    #[test]
    fn validate_keeps_order_and_reports_each_item() {
        let items = [3_u32, 4, 5, 6];
        let results = par_validate(&items, |v| if v % 2 == 0 { Ok(()) } else { Err(*v) });
        assert_eq!(results, [Err(3), Ok(()), Err(5), Ok(())]);
    }

    #[test]
    fn channel_delivers_and_drains_in_order() {
        let channel = JobChannel::bounded(8);
        channel.try_send("thumb-a").unwrap();
        channel.try_send("thumb-b").unwrap();
        assert_eq!(channel.drain(), vec!["thumb-a", "thumb-b"]);
        assert!(channel.drain().is_empty());
    }

    #[test]
    fn worker_thread_can_report_through_sender() {
        let channel = JobChannel::bounded(4);
        let sender = channel.sender();
        std::thread::spawn(move || {
            let sum = par_finite_sum(&[1.0, 2.0, 3.0]).unwrap();
            sender.send(sum).unwrap();
        })
        .join()
        .unwrap();
        assert_eq!(channel.drain(), vec![6.0]);
    }
}
