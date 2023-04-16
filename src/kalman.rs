use nalgebra::SMatrix;

type Estimate = SMatrix<f32, 2, 1>;
type Covariance = SMatrix<f32, 2, 2>;
type Prediction = SMatrix<f32, 2, 2>;
type Control = SMatrix<f32, 2, 1>;
type Noise = SMatrix<f32, 2, 2>;
type Measurement = SMatrix<f32, 2, 2>;
type Reading = SMatrix<f32, 2, 1>;

fn do_it() {
    // What we know
    let delta_t = 1.0;
    let acceleration = 0.5;

    // Uncertainty in our model, and in the sensors.
    let world_slop = Noise::from_diagonal_element(1.0);
    let sensor_noise = Noise::from_diagonal_element(0.5);

    // General setup
    let prediction = Prediction::new(1.0, delta_t, 0.0, 1.0);
    let control = Control::new((delta_t * delta_t) / 2.0, delta_t);
    let measurement = Measurement::identity();

    // Initial data
    let current_estimate = Estimate::new(1.0, 2.0);
    let covariance = Covariance::new(1.0, 0.0, 0.0, 1.0);

    // dbg!(current_estimate);
    // dbg!(covariance);

    // A prediction
    let next_estimate = prediction * current_estimate + control * acceleration;
    let next_covariance = prediction * covariance * prediction.transpose() + world_slop;

    // dbg!(next_estimate);
    // dbg!(next_covariance);

    // These are identical because the measurement matrix is an
    // identity matrix.
    let expected_readings = measurement * next_estimate;
    let expected_noise = measurement * next_covariance * measurement.transpose();

    // A sensor reading
    let sensor_reading = Reading::new(2.5, 2.0);

    // dbg!(sensor_reading);
    // dbg!(sensor_noise);

    // Update step
    let kalman_gain = next_covariance
        * measurement.transpose()
        * (expected_noise + sensor_noise).try_inverse().unwrap();
    let combined_estimate = next_estimate + kalman_gain * (sensor_reading - expected_readings);
    let combined_covariance = next_covariance - kalman_gain * measurement * next_covariance;

    // dbg!(combined_estimate);
    // dbg!(combined_covariance);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_out() {
        do_it();
        assert!(false);
    }
}
