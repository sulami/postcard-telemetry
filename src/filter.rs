//! Filters for data

use num_traits::float::Float;

/// Apply a high-pass filter to `seq` based on the time `dt` between
/// the data points and the RC constant `rc`.
///
/// **Important**: `dt` needs to be positive, non-zero, and normal
/// (finite and not `NaN`).
pub fn high_pass<T: Float>(seq: &mut [T], dt: T, rc: T) {
    if seq.is_empty() {
        return;
    }

    assert!(dt.is_sign_positive(), "dt needs to be > 0");
    assert!(dt.is_normal(), "dt needs to be normal");

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
///
/// **Important**: `dt` needs to be positive, non-zero, and normal
/// (finite and not `NaN`).
pub fn low_pass<T: Float>(seq: &mut [T], dt: T, rc: T) {
    if seq.is_empty() {
        return;
    }

    assert!(dt.is_sign_positive(), "dt needs to be > 0");
    assert!(dt.is_normal(), "dt needs to be normal");

    let alpha = dt / (rc + dt);
    seq[0] = seq[0] * alpha;

    for i in 1..seq.len() {
        seq[i] = seq[i - 1] + alpha * (seq[i] - seq[i - 1]);
    }
}

/// Helper function to apply a filter without mutating the original
/// slice.
pub fn apply_filter<T: Float, const N: usize>(
    filter: &dyn Fn(&mut [T], T, T),
    seq: &[T; N],
    dt: T,
    rc: T,
) -> [T; N] {
    let mut new = *seq;
    filter(&mut new, dt, rc);
    new
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

    #[test]
    fn new_filter_works() {
        let data = [9.81, 9.81, 9.81, 9.81, 9.81, 9.81, 9.81, 9.81];
        let new = apply_filter(&high_pass, &data, 20.0, 10.0);
        assert_eq!(*data.last().unwrap(), 9.81);
        assert_ne!(*new.last().unwrap(), 9.81)
    }
}
