//! Filters for data

use num_traits::float::Float;

/// Apply a high-pass filter to `seq` based on the time `dt` between
/// the data points and the RC constant `rc`.
pub fn high_pass<T: Float>(seq: &mut [T], dt: T, rc: T) {
    if seq.is_empty() {
        return;
    }

    assert!(dt.is_normal(), "dt needs to be > 0");

    let alpha = rc / (rc + dt);
    let mut previous_original = seq[0];

    for i in 1..seq.len() {
        let current_original = seq[i];
        seq[i] = alpha * (seq[i - 1] + seq[i] - previous_original);
        previous_original = current_original;
    }
}

/// Apply low-pass filter to `seq` based on the time `dt` between
/// the data points and the RC constant `rc`.
pub fn low_pass<T: Float>(seq: &mut [T], dt: T, rc: T) {
    if seq.is_empty() {
        return;
    }

    assert!(dt.is_normal(), "dt needs to be > 0");

    let alpha = dt / (rc + dt);
    seq[0] = seq[0] * alpha;

    for i in 1..seq.len() {
        seq[i] = seq[i - 1] + alpha * (seq[i] - seq[i - 1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn high_pass_works() {
        let mut data = [9.81, 9.81, 9.81, 9.81, 9.81, 9.81, 9.81, 9.81];
        high_pass(&mut data, 20.0, 10.0);
        assert!(*data.last().unwrap() < 0.01);
    }

    #[test]
    fn low_pass_works() {
        let mut data = [9.81, 9.81, 9.81, 9.81, 9.81, 9.81, 9.81, 9.81];
        low_pass(&mut data, 20.0, 10.0);
        assert!((*data.last().unwrap() - 9.81).abs() < 0.01);
    }
}
