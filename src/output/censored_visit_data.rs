use chrono::NaiveDateTime;
use serde::Serialize;
use crate::core::CensoredVisit;

#[derive(Serialize)]
struct CensoredVisitLine {
    in_time: NaiveDateTime,
    out_time_start: NaiveDateTime,
    out_time_end: NaiveDateTime,
    min_duration: usize,
    max_duration: usize,
    parked: bool,
}

impl From<&CensoredVisit> for CensoredVisitLine {
    fn from(visit: &CensoredVisit) -> Self {
        Self {
            in_time: visit.in_time,
            out_time_start: visit.out_time_start,
            out_time_end: visit.out_time_end,
            min_duration: visit.min_duration(),
            max_duration: visit.max_duration(),
            parked: visit.parked,
        }
    }
}

/// Writes visit data to "censored_visits.csv"
pub fn output_censored_visits(visits: &[CensoredVisit]) {
    let mut wtr = csv::Writer::from_path(
        "output_data/censored_visits.csv"
    ).unwrap();
    for visit in visits { 
        wtr.serialize(CensoredVisitLine::from(visit)).unwrap(); 
    }
    wtr.flush().unwrap();
    let mut wtr = csv::Writer::from_path(
        "../merged_data/censored_visits.csv"
    ).unwrap();
    for visit in visits { 
        wtr.serialize(CensoredVisitLine::from(visit)).unwrap(); 
    }
    wtr.flush().unwrap();
}

