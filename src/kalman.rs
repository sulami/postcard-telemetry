use nalgebra::SMatrix;

/// A classic Kalman filter with:
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

    /// Based in the static matrices, the current state estimate, the
    /// current control inputs, and a sensor reading generate a new
    /// state estimate.
    pub fn next(
        &self,
        current: &(SMatrix<f32, S, 1>, SMatrix<f32, S, S>),
        inputs: &SMatrix<f32, I, 1>,
        sensor_reading: &SMatrix<f32, R, 1>,
    ) -> (SMatrix<f32, S, 1>, SMatrix<f32, S, S>) {
        let (current_estimate, covariance) = current;

        // A prediction.
        let next_estimate = self.prediction * current_estimate + self.control * inputs;
        let next_covariance =
            self.prediction * covariance * self.prediction.transpose() + self.uncertainty;

        let expected_readings = self.measurement * next_estimate;
        let expected_noise = self.measurement * next_covariance * self.measurement.transpose();

        // Combine our prediction with our sensor reading.
        let kalman_gain = next_covariance
            * self.measurement.transpose()
            * (expected_noise + self.sensor_noise).try_inverse().unwrap();
        let combined_estimate = next_estimate + kalman_gain * (sensor_reading - expected_readings);
        let combined_covariance =
            next_covariance - kalman_gain * self.measurement * next_covariance;

        (combined_estimate, combined_covariance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use libc_print::std_name::dbg;
    use nalgebra::{Matrix2, Vector1, Vector2};

    #[test]
    fn try_out() {
        // Time between estimates.
        let delta_t = 1.0;

        // Active control inputs.
        let acceleration = Vector1::new(0.5);

        // Uncertainty in our model, and in the sensors.
        let world_slop = Matrix2::from_diagonal_element(1.0);
        let sensor_noise = Matrix2::from_diagonal_element(0.5);

        // How we expect the estimate to change.
        let prediction = Matrix2::new(1.0, delta_t, 0.0, 1.0);
        // How acceleration affects out estimate.
        let control = Vector2::new((delta_t * delta_t) / 2.0, delta_t);
        // How to convert from our model to expected sensor readings.
        let measurement = Matrix2::identity();

        // Initial estimate.
        let current_estimate = Vector2::new(1.0, 2.0);
        let covariance = Matrix2::new(1.0, 0.0, 0.0, 1.0);
        let sensor_reading = Vector2::new(2.5, 2.0);

        let filter = KalmanFilter::new(prediction, measurement, control, sensor_noise, world_slop);
        dbg!(filter.next(
            &(current_estimate, covariance),
            &acceleration,
            &sensor_reading
        ));
        assert!(false);
    }
}
