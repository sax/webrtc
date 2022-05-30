pub mod instant_to_epoch_ms {
    // Serializes a `tokio::time::Instant` to an approximation of epoch time in the form
    // of an `f64` where the integer portion is seconds and the decimal portion is milliseconds.
    //
    // Note that an `Instant` is not connected to real world time, so this conversion is
    // approximate.
    use serde::{Serialize, Serializer};
    use std::time::{SystemTime, UNIX_EPOCH};
    use tokio::time::Instant;

    pub fn serialize<S>(instant: &Instant, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let system_now = SystemTime::now();
        let instant_now = Instant::now();
        let approx = system_now - (instant_now - *instant);
        let epoch = approx
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        let epoch_ms = epoch.as_millis() as f64 / 1000.0;

        epoch_ms.serialize(serializer)
    }
}
