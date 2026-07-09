use std::iter::repeat;
use chrono::NaiveDateTime;
use serde::Deserialize;
use std::fs::File;
use std::io::{ BufRead, BufReader };
use crate::core::Timestamp;
use super::parsers::{ parse_optional_date, parse_optional_number };

/// Represents a row in the "clicker" CSV
#[derive(Debug, Deserialize)]
struct ClickerLine {
    #[serde(rename = "Start Time", deserialize_with = "parse_optional_date")]
    start_time: Option<NaiveDateTime>,
    #[serde(rename = "End Time", deserialize_with = "parse_optional_date")]
    end_time: Option<NaiveDateTime>, 
    #[serde(rename = "In", deserialize_with = "parse_optional_number")]
    in_count: Option<usize>,
    #[serde(rename = "Out", deserialize_with = "parse_optional_number")]
    out_count: Option<usize>,
}

/// Loads data from "clicker" CSV
fn read_clicker_file(path: &str) -> Vec<ClickerLine> {
    let mut reader = csv::Reader::from_path(path).unwrap();
    let mut rows = Vec::new();
    for result in reader.deserialize() {
        let line: ClickerLine = result.unwrap();
        rows.push(line);
    }
    rows
}

fn count_missing(path: &str) -> usize {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader.lines().count()
}


pub fn load_day(date: &str) -> (Vec<Timestamp>, Vec<Timestamp>) {
    let path = 
        "input_data/clicker_data/".to_string() 
        + date 
        + " - Clicker.csv";
    let data = read_clicker_file(&path);
    
    let missing_path = 
        "input_data/missing_card_data/".to_string() 
        + date 
        + " - Missing Cards.csv";
    // * added for days with incomplete clicker data
    let missing_count = count_missing(&missing_path) 
        * ((data.len() > 1) as usize);
    
    let day_start = NaiveDateTime::parse_from_str(
        &format!("2026-{} 04:00:00", date), "%Y-%m-%d %H:%M:%S"
    ).expect("Failed to parse datetime"); 
    let day_end = NaiveDateTime::parse_from_str(
        &format!("2026-{} 22:00:00", date), "%Y-%m-%d %H:%M:%S"
    ).expect("Failed to parse datetime");
    
    let mut in_timestamps = Vec::<Timestamp>::new();
    let mut out_timestamps = Vec::<Timestamp>::new();
    for line in &data {
        let timestamp = match (line.start_time, line.end_time) {
            (None, None) => Timestamp { 
                start: day_start, 
                end: day_end
            },
            (None, Some(out_time)) => Timestamp { 
                start: day_start, 
                end: out_time,
            },
            (Some(in_time), None) => Timestamp { 
                start: in_time,
                end: day_end,
            },
            (Some(in_time), Some(out_time)) => Timestamp { 
                start: in_time, 
                end: out_time,
            },
        };
        in_timestamps.extend(
            repeat(timestamp).take(line.in_count.unwrap_or(0))
        );
        out_timestamps.extend(
            repeat(timestamp).take(line.out_count.unwrap_or(0))
        );
    } 
    if let Some(last_line) = data.last() {
        let remaining_cars = in_timestamps.len()
            .saturating_sub(out_timestamps.len())
            .saturating_sub(missing_count);
        let timestamp = last_line.end_time.unwrap();
        for _ in 0..remaining_cars { 
            out_timestamps.push(Timestamp{ 
                start: timestamp,
                end: day_end,
            }); 
        }
    }
    (in_timestamps, out_timestamps)
}
