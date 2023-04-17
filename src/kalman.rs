use nalgebra::SMatrix;

type State = SMatrix<f32, 2, 1>;
type Covariance = SMatrix<f32, 2, 2>;
type Prediction = SMatrix<f32, 2, 2>;
type Control = SMatrix<f32, 2, 1>;
type Noise = SMatrix<f32, 2, 2>;
type Measurement = SMatrix<f32, 2, 2>;
type Reading = SMatrix<f32, 2, 1>;
type Inputs = SMatrix<f32, 1, 1>;

type Estimate = (State, Covariance);

/// A classic Kalman filter.
pub struct KalmanFilter {
    prediction: Prediction,
    measurement: Measurement,
    control: Control,
    sensor_noise: Noise,
    uncertainty: Noise,
}

impl KalmanFilter {
    /// Construct a new filter. Takes a few static matrices that it
    /// holds on to.
    pub fn new(
        prediction: Prediction,
        measurement: Measurement,
        control: Control,
        sensor_noise: Noise,
        uncertainty: Noise,
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
    pub fn next(&self, current: &Estimate, inputs: &Inputs, sensor_reading: &Reading) -> Estimate {
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

    #[test]
    fn try_out() {
        // Time between estimates.
        let delta_t = 1.0;

        // Active control inputs.
        let acceleration = Inputs::new(0.5);

        // Uncertainty in our model, and in the sensors.
        let world_slop = Noise::from_diagonal_element(1.0);
        let sensor_noise = Noise::from_diagonal_element(0.5);

        // How we expect the estimate to change.
        let prediction = Prediction::new(1.0, delta_t, 0.0, 1.0);
        // How acceleration affects out estimate.
        let control = Control::new((delta_t * delta_t) / 2.0, delta_t);
        // How to convert from our model to expected sensor readings.
        let measurement = Measurement::identity();

        // Initial estimate.
        let current_estimate = State::new(1.0, 2.0);
        let covariance = Covariance::new(1.0, 0.0, 0.0, 1.0);
        let sensor_reading = Reading::new(2.5, 2.0);

        let filter = KalmanFilter::new(prediction, measurement, control, sensor_noise, world_slop);
        dbg!(filter.next(
            &(current_estimate, covariance),
            &acceleration,
            &sensor_reading
        ));
        assert!(false);
    }
}
