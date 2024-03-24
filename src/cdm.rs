use crate::*;
use chrono::prelude::*;
use linfa::prelude::*;
use linfa_linear::LinearRegression;
use ndarray::{Array1, Array2, Axis};

pub struct TimeDriftMethod {
    past_measurements: Vec<(f64, f64)>,
    max_clock_drift_dev: f64,
    measurement_count: usize,
}

impl TimeDriftMethod {
    pub fn new(max_clock_drift_dev: f64) -> Self {
        Self {
            past_measurements: Vec::new(),
            max_clock_drift_dev,
            measurement_count: 0,
        }
    }

    pub fn detect_spoofing_attack(
        &mut self,
        local_system_time: DateTime<Utc>,
        gps_time: DateTime<Utc>,
    ) -> bool {
        let clock_drift = local_system_time.timestamp() - gps_time.timestamp();
        self.past_measurements
            .push((clock_drift as f64, local_system_time.timestamp() as f64));
        self.measurement_count += 1;

        if self.past_measurements.len() < 2 {
            return false;
        }

        let max_past_measurements = self.past_measurements.len();

        let (x, y): (Vec<_>, Vec<_>) = self.past_measurements.iter().cloned().unzip();

        let mut y_noisy = y.clone();

        for y_val in y_noisy.iter_mut() {
            *y_val += rand::random::<f64>() * 0.0001 + 0.000001;
        }

        let data = Array2::from_shape_vec((max_past_measurements, 1), x).unwrap();
        let targets = Array1::from(y_noisy);

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
