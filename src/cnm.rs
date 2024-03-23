use crate::utilities::Satellite;
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

    fn detect_spoofing_attack(&self, satellites: &[Satellite]) -> bool {
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
