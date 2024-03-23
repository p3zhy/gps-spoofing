use chrono::prelude::*;
use linfa::prelude::*;
use linfa_linear::LinearRegression;
use ndarray::{Array1, Array2, Axis};

pub struct TimeDriftMethod<const N: usize> {
    past_measurements: [(f64, f64); N],

    max_clock_drift_dev: f64,
    initial_time: DateTime<Utc>,
    measurement_count: usize,
}

impl<const N: usize> TimeDriftMethod<N> {
    pub fn new(max_clock_drift_dev: f64, initial_time: DateTime<Utc>) -> Self {
        Self {
            past_measurements: [(0.0, 0.0); N],
            max_clock_drift_dev,
            initial_time,
            measurement_count: 0,
        }
    }

    pub fn detect_spoofing_attack(
        &mut self,
        local_system_time: DateTime<Utc>,
        gps_time: DateTime<Utc>,
    ) -> bool {
        let max_past_measurements = self.past_measurements.len();
        let clock_drift = local_system_time.timestamp() - gps_time.timestamp();
        let index = self.measurement_count % max_past_measurements;
        self.past_measurements[index] = (clock_drift as f64, local_system_time.timestamp() as f64);
        self.measurement_count += 1;

        if self.measurement_count < max_past_measurements {
            return false;
        }

        let mut x = [0.0; N];
        let mut y = [0.0; N];
        for i in 0..max_past_measurements {
            let (drift, time) = self.past_measurements[(index + i) % max_past_measurements];
            x[i] = drift;
            y[i] = time;
        }

        let mut y_noisy = y;

        for y_val in y_noisy.iter_mut() {
            *y_val += rand::random::<f64>() * 0.0001 + 0.000001;
        }

        let data = Array2::from_shape_vec((max_past_measurements, 1), x.to_vec()).unwrap();
        let targets = Array1::from(y_noisy.to_vec());

        let dataset = Dataset::new(data, targets);

        let lin_reg = LinearRegression::new();
        let model = lin_reg.fit(&dataset).unwrap();

        let y_pred = model.predict(&dataset);
        let loss = (dataset.targets() - &y_pred.insert_axis(Axis(1)))
            .mapv(|x| x.abs())
            .mean();
        loss > Some(self.max_clock_drift_dev)
    }
}
