use crate::*;
use chrono::prelude::*;
use utilities::*;
const TLE_FILE_CONTENTS: &str = include_str!("./../gps.tle");

struct TwoLineElement {
    pseudo_random_noise: u16,
    satellite_record: satellite::io::Satrec,
}

impl TwoLineElement {
    fn new(pseudo_random_noise: u16, line1: &str, line2: &str) -> TwoLineElement {
        let satellite_record = satellite::io::twoline2satrec(line1, line2).unwrap();
        TwoLineElement {
            pseudo_random_noise,
            satellite_record,
        }
    }

    fn observer_view(
        &self,
        time: DateTime<Utc>,
        latitude: f32,
        longitude: f32,
        height: f32,
    ) -> (f32, f32) {
        let result =
            satellite::propogation::propogate_datetime(&self.satellite_record, time).unwrap();

        let observer = satellite::Geodedic {
            longitude: longitude * satellite::constants::DEG_2_RAD,
            latitude: latitude * satellite::constants::DEG_2_RAD,
            height,
        };

        let gmst = satellite::propogation::gstime::gstime_datetime(time);
        let position_ecf = satellite::transforms::eci_to_ecf(&result.position, gmst);
        let look_angles = satellite::transforms::ecf_to_look_angles(&observer, &position_ecf);

        (look_angles.elevation, look_angles.azimuth)
    }
}

struct OrbitPositionsMethod {
    two_line_elements: Vec<TwoLineElement>,
    min_elevation: u8,
    allowed_azimuth_deviation: f32,
    allowed_elevation_deviation: f32,
}

impl OrbitPositionsMethod {
    fn new(
        min_elevation: u8,
        allowed_azimuth_deviation: f32,
        allowed_elevation_deviation: f32,
    ) -> OrbitPositionsMethod {
        let mut method = OrbitPositionsMethod {
            two_line_elements: Vec::new(),
            min_elevation,
            allowed_azimuth_deviation,
            allowed_elevation_deviation,
        };
        method.load_two_line_elements();
        method
    }

    fn load_two_line_elements(&mut self) {
        let lines: Vec<&str> = TLE_FILE_CONTENTS.split('\n').collect();
        for chunk in lines.chunks(3) {
            let pseudo_random_noise = chunk.get(0).unwrap().parse::<u16>().unwrap();
            let line1 = chunk.get(1).unwrap_or(&"");
            let line2 = chunk.get(2).unwrap_or(&"");
            self.two_line_elements
                .push(TwoLineElement::new(pseudo_random_noise, line1, line2));
        }
    }

    fn detect_spoofing_attack(
        &mut self,
        local_system_time: DateTime<Utc>,
        satellites: &[Satellite],
        latitude: f32,
        longitude: f32,
        height: f32,
    ) -> bool {
        for satellite in satellites {
            if let Some(two_line_element) = self.get_two_line_element(satellite.pseudo_random_noise)
            {
                let (elevation, azimuth) =
                    two_line_element.observer_view(local_system_time, latitude, longitude, height);
                let azimuth_diff = minimum_angle_difference(azimuth, satellite.azimuth as f32);
                let elevation_diff =
                    minimum_angle_difference(elevation, satellite.elevation as f32);
                if satellite.elevation < self.min_elevation
                    || azimuth_diff > self.allowed_azimuth_deviation
                    || elevation_diff > self.allowed_elevation_deviation
                {
                    return true;
                }
            }
        }
        false
    }

    fn get_two_line_element(&self, pseudo_random_noise: u16) -> Option<&TwoLineElement> {
        self.two_line_elements
            .iter()
            .find(|&tle| tle.pseudo_random_noise == pseudo_random_noise)
    }
}
