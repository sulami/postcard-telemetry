//! Kalman filters
//!
//! This filter implementation works with arbitrary dimensions, but is
//! restricted to [`f32`] for the time being.
//!
//! It is also restricted to constant time intervals between
//! iterations.

use nalgebra::SMatrix;

/// A classic Kalman filter
///
/// The following dimensions can be decided on:
/// - S state variables
/// - I active inputs
/// - R sensor readings
///
/// Based on those dimensions, all other dimensions are fixed.
pub struct KalmanFilter<const S: usize, const I: usize, const R: usize> {
    prediction: SMatrix<f32, S, S>,
    measurement: SMatrix<f32, R, S>,
    control: SMatrix<f32, S, I>,
    sensor_noise: SMatrix<f32, R, R>,
    uncertainty: SMatrix<f32, S, S>,
}

impl<const S: usize, const I: usize, const R: usize> KalmanFilter<S, I, R> {
    /// Construct a new filter. Takes a few static matrices that it
    /// holds on to.
    pub fn new(
        prediction: SMatrix<f32, S, S>,
        measurement: SMatrix<f32, R, S>,
        control: SMatrix<f32, S, I>,
        sensor_noise: SMatrix<f32, R, R>,
        uncertainty: SMatrix<f32, S, S>,
    ) -> Self {
        Self {
            prediction,
            measurement,
            control,
            sensor_noise,
            uncertainty,
        }
    }

    /// Run the initial predection cycle at t = 0;
    pub fn init(
        &self,
        (previous_estimate, previous_covariance): &(SMatrix<f32, S, 1>, SMatrix<f32, S, S>),
    ) -> (SMatrix<f32, S, 1>, SMatrix<f32, S, S>) {
        let next_estimate =
            self.prediction * previous_estimate + self.control * SMatrix::<f32, I, 1>::zeros();
        let next_covariance =
            self.prediction * previous_covariance * self.prediction.transpose() + self.uncertainty;
        (next_estimate, next_covariance)
    }

    /// Based in the static matrices, the current state estimate, the
    /// current control inputs, and a sensor reading generate a new
    /// state estimate.
    pub fn next(
        &self,
        (previous_estimate, previous_covariance): &(SMatrix<f32, S, 1>, SMatrix<f32, S, S>),
        inputs: &SMatrix<f32, I, 1>,
        sensor_readings: &SMatrix<f32, R, 1>,
    ) -> (SMatrix<f32, S, 1>, SMatrix<f32, S, S>) {
        let kalman_gain = previous_covariance
            * self.measurement.transpose()
            * (self.measurement * previous_covariance * self.measurement.transpose()
                + self.sensor_noise)
                .try_inverse()
                .unwrap();

        // A prediction.
        let current_estimate = previous_estimate
            + kalman_gain * (sensor_readings - self.measurement * previous_estimate);
        let current_covariance = (SMatrix::<f32, S, S>::identity()
            - kalman_gain * self.measurement)
            * previous_covariance;

        let predicted_state = self.prediction * current_estimate + self.control * inputs;
        let predicted_covariance =
            self.prediction * current_covariance * self.prediction.transpose() + self.uncertainty;
        (predicted_state, predicted_covariance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use nalgebra::{Matrix2, Matrix6, Vector1, Vector2, Vector6};

    #[test]
    fn test_kalman_filter_works() {
        // Numerical example 10 (rocket) taken from
        // https://www.kalmanfilter.net/multiExamples.html

        // We have a rocket with constant acleration, and our state is
        // comprised of altitude and velocity.

        // Time between estimates.
        let delta_t = 0.25;

        // How we expect the estimate to change.
        // Also known as the state trasition matrix F.
        let prediction = Matrix2::new(1.0, delta_t, 0.0, 1.0);
        // How acceleration affects out estimate, control matrix G.
        let control = Vector2::new(0.5 * delta_t * delta_t, delta_t);
        // How to convert from our model to expected sensor readings.
        let measurement = SMatrix::<f32, 1, 2>::new(1.0, 0.0);

        // Start with high uncertainty because our initial estimate is
        // a guess. This is the process noise matrix Q.
        let estimate_uncertainty = Matrix2::new(
            0.25 * delta_t * delta_t * delta_t * delta_t,
            0.5 * delta_t * delta_t * delta_t,
            0.5 * delta_t * delta_t * delta_t,
            delta_t * delta_t,
        ) * 0.01;

        // Measurement variance R.
        let sensor_noise = Vector1::new(400.);

        // Initial estimate. Bias towards measurements because we're
        // guessing the initial state.
        let current_estimate = Vector2::zeros();
        let covariance = Matrix2::new(500.0, 0.0, 0.0, 500.00);

        let filter = KalmanFilter::new(
            prediction,
            measurement,
            control,
            sensor_noise,
            estimate_uncertainty,
        );

        // Active control inputs and sensor readings.
        // The control input is the acceleration, minus gravity.
        let frames = [
            (Vector1::new(39.81 - 9.81), Vector1::new(6.43)),
            (Vector1::new(39.67 - 9.81), Vector1::new(1.3)),
            (Vector1::new(39.81 - 9.81), Vector1::new(39.43)),
            (Vector1::new(39.84 - 9.81), Vector1::new(45.89)),
            (Vector1::new(40.05 - 9.81), Vector1::new(41.44)),
            (Vector1::new(39.85 - 9.81), Vector1::new(48.7)),
            (Vector1::new(39.78 - 9.81), Vector1::new(78.06)),
            (Vector1::new(39.65 - 9.81), Vector1::new(80.08)),
            (Vector1::new(39.67 - 9.81), Vector1::new(61.77)),
            (Vector1::new(39.78 - 9.81), Vector1::new(75.15)),
            (Vector1::new(39.59 - 9.81), Vector1::new(110.39)),
            (Vector1::new(39.87 - 9.81), Vector1::new(127.83)),
            (Vector1::new(39.85 - 9.81), Vector1::new(158.75)),
            (Vector1::new(39.59 - 9.81), Vector1::new(156.55)),
            (Vector1::new(39.84 - 9.81), Vector1::new(213.32)),
            (Vector1::new(39.90 - 9.81), Vector1::new(229.82)),
            (Vector1::new(39.63 - 9.81), Vector1::new(262.8)),
            (Vector1::new(39.59 - 9.81), Vector1::new(297.57)),
            (Vector1::new(39.76 - 9.81), Vector1::new(335.69)),
            (Vector1::new(39.79 - 9.81), Vector1::new(367.92)),
            (Vector1::new(39.73 - 9.81), Vector1::new(377.19)),
            (Vector1::new(39.93 - 9.81), Vector1::new(411.18)),
            (Vector1::new(39.83 - 9.81), Vector1::new(460.7)),
            (Vector1::new(39.85 - 9.81), Vector1::new(468.39)),
            (Vector1::new(39.94 - 9.81), Vector1::new(553.9)),
            (Vector1::new(39.86 - 9.81), Vector1::new(583.97)),
            (Vector1::new(39.76 - 9.81), Vector1::new(655.15)),
            (Vector1::new(39.86 - 9.81), Vector1::new(723.09)),
            (Vector1::new(39.74 - 9.81), Vector1::new(736.85)),
            (Vector1::new(39.94 - 9.81), Vector1::new(787.22)),
        ];

        let mut current = (current_estimate, covariance);

        // Zeroeth iteration.
        current = filter.init(&current);

        // First iteration.
        current = filter.next(&current, &frames[0].0, &frames[0].1);

        // Second iteration.
        current = filter.next(&current, &frames[1].0, &frames[1].1);

        // Remaining iterations.
        for (input, readings) in frames.iter().skip(2) {
            current = filter.next(&current, &input, &readings);
        }

        let (estimate, _) = current;

        assert!((estimate.x - 851.9).abs() < 0.1);
        assert!((estimate.y - 223.2).abs() < 0.1);
    }

    #[test]
    fn test_kalman_filter_six_dimensional() {
        // Same as above, but with the other example, a car in 2D.

        // k is now/next
        // ^x_k-1 = previous_estimate
        // ^x_k = estimate
        // P_k = covariance
        // F_k = prediction/transition matrix
        // B_k = control matrix
        // u_k = control inputs
        // Q_k = uncertainty
        // H_k = measurement matrix
        // R_k = sensor noise
        let control_error_stddev = 0.2;
        let sensor_error_stddev = 3.;
        let delta_t = 1.0;
        let f = Matrix6::new(
            1.,
            delta_t,
            0.5 * delta_t * delta_t,
            0.,
            0.,
            0.,
            0.,
            1.,
            delta_t,
            0.,
            0.,
            0.,
            0.,
            0.,
            1.,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
            1.,
            delta_t,
            0.5 * delta_t * delta_t,
            0.,
            0.,
            0.,
            0.,
            1.,
            delta_t,
            0.,
            0.,
            0.,
            0.,
            0.,
            1.,
        );
        let q = Matrix6::new(
            0.25 * delta_t * delta_t * delta_t * delta_t,
            0.5 * delta_t * delta_t * delta_t,
            0.5 * delta_t * delta_t,
            0.,
            0.,
            0.,
            0.5 * delta_t * delta_t * delta_t,
            delta_t * delta_t,
            delta_t,
            0.,
            0.,
            0.,
            0.5 * delta_t * delta_t,
            delta_t,
            1.,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.25 * delta_t * delta_t * delta_t * delta_t,
            0.5 * delta_t * delta_t * delta_t,
            0.5 * delta_t * delta_t,
            0.,
            0.,
            0.,
            0.5 * delta_t * delta_t * delta_t,
            delta_t * delta_t,
            delta_t,
            0.,
            0.,
            0.,
            0.5 * delta_t * delta_t,
            delta_t,
            1.,
        ) * control_error_stddev
            * control_error_stddev;
        let r = Matrix2::from_diagonal_element(sensor_error_stddev * sensor_error_stddev);
        let current_estimate = Vector6::zeros();
        let covariance = Matrix6::from_diagonal_element(500.);
        let h = SMatrix::<f32, 2, 6>::new(1., 0., 0., 0., 0., 0., 0., 0., 0., 1., 0., 0.);
        let b = Matrix6::identity();
        let filter = KalmanFilter::new(f, h, b, r, q);

        let mut current = (current_estimate, covariance);
        current = filter.init(&current);

        let frames = [
            Vector2::new(301.5, -401.46),
            Vector2::new(298.23, -375.44),
            Vector2::new(297.83, -346.15),
            Vector2::new(300.42, -320.2),
            Vector2::new(301.94, -300.08),
            Vector2::new(299.5, -274.12),
            Vector2::new(305.98, -253.45),
            Vector2::new(301.25, -226.4),
            Vector2::new(299.73, -200.65),
            Vector2::new(299.2, -171.62),
            Vector2::new(298.62, -152.11),
            Vector2::new(301.84, -125.19),
            Vector2::new(299.6, -93.4),
            Vector2::new(295.3, -74.79),
            Vector2::new(299.3, -49.12),
            Vector2::new(301.95, -28.73),
            Vector2::new(296.3, 2.99),
            Vector2::new(295.11, 25.65),
            Vector2::new(295.12, 49.86),
            Vector2::new(289.9, 72.87),
            Vector2::new(283.51, 96.34),
            Vector2::new(276.42, 120.4),
            Vector2::new(264.22, 144.69),
            Vector2::new(250.25, 168.06),
            Vector2::new(236.66, 184.99),
            Vector2::new(217.47, 205.11),
            Vector2::new(199.75, 221.82),
            Vector2::new(179.7, 238.3),
            Vector2::new(160., 253.02),
            Vector2::new(140.92, 267.19),
            Vector2::new(113.53, 270.71),
            Vector2::new(93.68, 285.86),
            Vector2::new(69.71, 292.9),
            Vector2::new(45.93, 298.77),
            Vector2::new(20.87, 298.77),
        ];

        for readings in frames {
            let inputs = Vector6::zeros();
            current = filter.next(&current, &inputs, &readings);
        }

        // NB There are some rounding errors here, but they are small
        // enough to be ignored.
        assert!((current.0[0] - -7.05).abs() < 0.1, "{}", current.0[0]);
        assert!((current.0[1] - -26.73).abs() < 0.1, "{}", current.0[1]);
        assert!((current.0[2] - -0.74).abs() < 0.1, "{}", current.0[2]);
        assert!((current.0[3] - 298.89).abs() < 4.0, "{}", current.0[3]);
        assert!((current.0[4] - 0.17).abs() < 0.6, "{}", current.0[4]);
        assert!((current.0[5] - -1.87).abs() < 0.1, "{}", current.0[5]);
    }
}
