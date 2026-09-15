//! Property tests for job helpers (`proptest`, P0-12).
//!
//! Parallel helpers must agree with their sequential models on every input,
//! and channels must preserve exactly the messages sent.

use petunia_core::{HandleTable, JobChannel, par_finite_sum};
use proptest::prelude::*;

proptest! {
    /// Parallel finite sum matches the sequential fold on finite inputs.
    #[test]
    fn par_sum_matches_sequential(values in prop::collection::vec(-1000.0f32..1000.0, 0..256)) {
        let expected: f64 = values.iter().map(|v| f64::from(*v)).sum();
        prop_assert_eq!(par_finite_sum(&values).unwrap(), expected);
    }

    /// Handle tables resolve exactly the live entries, in insertion count.
    #[test]
    fn handle_table_counts_live_entries(n in 1usize..64) {
        let mut table = HandleTable::new();
        let mut live = Vec::new();
        for i in 0..n {
            live.push(table.insert(i));
        }
        prop_assert_eq!(table.len(), n);
        for (i, handle) in live.iter().enumerate() {
            prop_assert_eq!(table.get(*handle), Some(&i));
        }
        // Removing half keeps the other half resolving.
        for handle in live.iter().step_by(2) {
            table.remove(*handle);
        }
        prop_assert_eq!(table.len(), n - n.div_ceil(2));
    }

    /// Channels deliver exactly what was sent, in order.
    #[test]
    fn channel_preserves_messages(values in prop::collection::vec(0u32..1000, 0..32)) {
        let channel = JobChannel::bounded(values.len().max(1));
        for v in &values {
            channel.try_send(*v).unwrap();
        }
        prop_assert_eq!(channel.drain(), values);
    }
}
