struct SatelliteState {
    carrier_to_noise_density: u8,
}

impl SatelliteState {
    fn new(carrier_to_noise_density: u8) -> Self {
        Self {
            carrier_to_noise_density,
        }
    }
}
struct CarrierToNoiseDensityMethod {
    min_carrier_to_noise_density: u8,
    max_carrier_to_noise_density: u8,
}

impl CarrierToNoiseDensityMethod {
    fn new(min_carrier_to_noise_density: u8, max_carrier_to_noise_density: u8) -> Self {
        CarrierToNoiseDensityMethod {
            min_carrier_to_noise_density,
            max_carrier_to_noise_density,
        }
    }

    fn detect_spoofing_attack(&self, satellites: &[SatelliteState]) -> bool {
        for satellite in satellites {
            if satellite.carrier_to_noise_density < self.min_carrier_to_noise_density
                || satellite.carrier_to_noise_density > self.max_carrier_to_noise_density
            {
                return true;
            }
        }
        false
    }
}
