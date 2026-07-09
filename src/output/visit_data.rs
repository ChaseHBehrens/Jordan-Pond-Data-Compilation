use chrono::NaiveDateTime;
use serde::Serialize;
use crate::core::{ Activity, Visit };

#[derive(Serialize)]
struct VisitLine {
    in_time: NaiveDateTime,
    out_time: NaiveDateTime,
    duration: usize,
    parked: bool,
    people_in_car: u8,
    visited_jordan_pond: bool,
    visited_jordan_pond_house: bool,
    visited_other_trails: bool,
    weather: String,
}

impl From<&Visit> for VisitLine {
    fn from(visit: &Visit) -> Self {
        Self {
            in_time: visit.in_time,
            out_time: visit.out_time,
            duration: visit.duration(),
            parked: visit.parked,
            people_in_car: visit.people_in_car,
            visited_jordan_pond: visit.activities
                .contains(&Activity::JordanPond),
            visited_jordan_pond_house: visit.activities
                .contains(&Activity::JordanPondHouse),
            visited_other_trails: visit.activities
                .contains(&Activity::Trails),
            weather: visit.weather.to_string(),
        }
    }
}

/// Writes visit data to "visits.csv"
pub fn output_visits(visits: &[Visit]) {
    let mut wtr = csv::Writer::from_path(
        "output_data/visits.csv"
    ).unwrap();
    for visit in visits { 
        wtr.serialize(VisitLine::from(visit)).unwrap(); 
    }
    wtr.flush().unwrap();
    let mut wtr = csv::Writer::from_path(
        "../merged_data/visits.csv"
    ).unwrap();
    for visit in visits { 
        wtr.serialize(VisitLine::from(visit)).unwrap(); 
    }
    wtr.flush().unwrap();
}

