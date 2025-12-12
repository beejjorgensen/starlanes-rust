/// Safely add an i64 to a u64
fn apply_delta(value: u64, delta: i64) -> u64 {
    if delta >= 0 {
        value.saturating_add(delta as u64)
    } else {
        value.saturating_sub((-delta) as u64)
    }
}
