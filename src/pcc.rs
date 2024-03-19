// The physical speed limit method or Physical Cross-Check for Speed Over Ground (PCC_sog)
//  validates that the vessel stays in reasonable speed ranges.

use crate::utilities;
use core::time::Duration;
pub struct PhysicalSpeedLimit {
    max_speed: f32,
}

impl PhysicalSpeedLimit {
    pub fn new(max_speed: f32) -> Self {
        Self { max_speed }
    }
    pub fn is_spoofing_attack(&self, speed: f32) -> bool {
        speed > self.max_speed
    }
}

// The physical height limit method or Physical Cross-Check for Height (PCC_height) validates that the altitude
//  of the vessel is at sea level.

pub struct PhysicalHeightLimit {
    max_height: f32,
    min_height: f32,
}

impl PhysicalHeightLimit {
    pub fn new(max_height: f32, min_height: f32) -> Self {
        Self {
            max_height,
            min_height,
        }
    }
    pub fn is_spoofing_attack(&self, height: f32) -> bool {
        height > self.max_height || height < self.min_height
    }
}

pub struct PhysicalRateOfTurnLimit {
    max_rate_of_turn: f32,
    min_speed_to_determine_rate_of_turn: f32,
    previous_update_time: Duration,
    previous_course: f32,
}

impl PhysicalRateOfTurnLimit {
    fn new(max_rate_of_turn: f32, min_speed_to_determine_rate_of_turn: f32) -> Self {
        Self {
            max_rate_of_turn,
            min_speed_to_determine_rate_of_turn,
            previous_update_time: Duration::default(),
            previous_course: 0.0,
        }
    }
    fn is_spoofing_attack(&mut self, speed: f32, update_time: Duration, course: f32) -> bool {
        if speed < self.min_speed_to_determine_rate_of_turn {
            return false;
        }

        let delta = (self.previous_update_time - update_time).as_secs_f32();
        let course_difference = utilities::minimum_angle_difference(course, self.previous_course);
        self.previous_course = course;
        self.previous_update_time = update_time;
        let rate_of_turn = (course_difference / delta).abs();
        if rate_of_turn > self.max_rate_of_turn {
            return true;
        }
        false
    }
}

pub struct PhysicalEnvironmentLimitMethod<const M: usize, const N: usize> {
    pub polygons: [Polygon<N>; M],
}

impl<const M: usize, const N: usize> PhysicalEnvironmentLimitMethod<M, N> {
    fn new(polygons: [Polygon<N>; M]) -> Self {
        Self { polygons }
    }
    pub fn is_spoofing_attack(&self, point: Point) -> bool {
        for polygon in &self.polygons {
            if polygon.is_inside(&point) {
                return true;
            }
        }
        false
    }
}

#[derive(Debug)]
pub struct Point {
    pub latitude: f32,
    pub longitude: f32,
}

impl Point {
    fn new(latitude: f32, longitude: f32) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}

#[derive(Debug)]
pub struct Polygon<const N: usize> {
    pub points: [Point; N],
    multipliers: [f32; N],
    constants: [f32; N],
}

impl<const N: usize> Polygon<N> {
    fn new(points: [Point; N]) -> Self {
        let n = points.len();
        let mut j = n - 1;
        let mut multipliers: [f32; N] = [0.0; N]; // Initialize arrays with default values
        let mut constants: [f32; N] = [0.0; N];
        for i in 0..points.len() {
            if points[j].latitude == points[i].latitude {
                constants[i] = points[i].longitude;
                multipliers[i] = 0.0;
            } else {
                constants[i] = points[i].longitude
                    - (points[i].latitude * points[j].longitude)
                        / (points[j].latitude - points[i].latitude)
                    + (points[i].latitude * points[i].longitude)
                        / (points[j].latitude - points[i].latitude);
                multipliers[i] = (points[j].longitude - points[i].longitude)
                    / (points[j].latitude - points[i].latitude);
            }
            j = i;
        }
        Self {
            points,
            multipliers,
            constants,
        }
    }
    pub fn is_inside(&self, point: &Point) -> bool {
        let mut odd_nodes = false;
        let n = self.points.len();
        let mut current: bool = self.points[n - 1].latitude > point.latitude;
        let mut prv: bool;

        for i in 0..self.points.len() {
            prv = current;
            current = self.points[i].latitude > point.latitude;
            if current != prv {
                odd_nodes ^=
                    point.latitude * self.multipliers[i] + self.constants[i] < point.longitude;
            }
        }
        return odd_nodes;
    }
}
