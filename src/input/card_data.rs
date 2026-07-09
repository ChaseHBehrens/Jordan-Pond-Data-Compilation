use std::{ array, collections::hash_map::HashMap, fs, str::FromStr };
use chrono::NaiveDateTime;
use serde::Deserialize;
use crate::core::{ Activity, DataTimestamp, ResearchTime, Timestamp, Visit, CensoredVisit, Weather };
use super::parsers::{ parse_bool, parse_date };


const TOTAL_NUMBER_OF_CARDS: usize = 300;


/// Represents a row in the "in" CSV
#[derive(Deserialize)]
struct InLine {
    #[serde(rename = "QR Number")]
    code: usize,
    #[serde(rename = "Timestamp", deserialize_with = "parse_date")]
    time: NaiveDateTime,
}

/// Loads data from "in" CSV
fn read_in_file(path: &str) -> Vec<InLine> {
    let mut reader = csv::Reader::from_path(path).unwrap();
    let mut rows = Vec::new();
    for result in reader.deserialize() {
        let line: InLine = result.unwrap();
        rows.push(line);
    }
    rows
}


/// Represents a row in the "out" CSV
#[derive(Deserialize)]
struct OutLine {
    #[serde(rename = "QR Number")]
    code: usize,
    #[serde(rename = "Timestamp", deserialize_with = "parse_date")]
    time: NaiveDateTime,
    #[serde(rename = "Parked?", deserialize_with = "parse_bool")]
    parked: bool,
    #[serde(rename = "# People in Car")]
    people_in_car: u8,
    #[serde(rename = "Where did They Go?")]
    activities: String,
}

/// Loads data from "out" CSV
fn read_out_file(path: &str) -> Vec<OutLine> {
    let mut reader = csv::Reader::from_path(path).unwrap();
    let mut rows = Vec::new();
    for result in reader.deserialize() {
        let line: OutLine = result.unwrap();
        rows.push(line);
    }
    rows
}


/// Represents if a card was missing at the end of the day
/// `Missing` hold the time we left at the end of the day
pub enum Missing {
    Yes,
    No(NaiveDateTime),
}

/// Loads data from "missing cards" CSV
fn read_missing_cards_file(path: &str) -> Vec<usize> {
    let mut reader = csv::Reader::from_path(path).unwrap();
    let mut rows = Vec::new();
    for result in reader.deserialize() {
        let line: usize = result.unwrap();
        rows.push(line);
    }
    rows
}


/// Represents the "card distribution time" CSV
#[derive(Deserialize)]
struct CardDistributionTime {
    in_start_time: NaiveDateTime,
    out_start_time: NaiveDateTime,
    in_end_time: NaiveDateTime,
    out_end_time: NaiveDateTime,
}

/// Loads data from "card distribution time" CSV
fn read_card_distribution_time_file(path: &str) -> CardDistributionTime {
    let mut reader = csv::Reader::from_path(path).unwrap();
    let mut times: HashMap<String, (NaiveDateTime, NaiveDateTime)> = 
        std::collections::HashMap::new();
    for result in reader.records() {
        let record = result.unwrap();
        let label = record.get(0).unwrap().to_string();    
        let start_str = record
            .get(1)
            .filter(|s| !s.trim().is_empty())
            .unwrap_or("01/01/1970 12:00 AM");
        let end_str = record
            .get(2)
            .filter(|s| !s.trim().is_empty())
            .unwrap_or("01/01/1970 12:00 AM");
        let start = NaiveDateTime::parse_from_str(
            start_str, "%m/%d/%Y %I:%M %p"
        ).unwrap();
        let end = NaiveDateTime::parse_from_str(
            end_str, "%m/%d/%Y %I:%M %p"
        ).unwrap(); 
        times.insert(label, (start, end));
    }
    let (in_start_time, in_end_time) = times["In"];
    let (out_start_time, out_end_time) = times["Out"];

    CardDistributionTime { 
        in_start_time, 
        out_start_time, 
        in_end_time, 
        out_end_time, 
    }
}


/// Finds all the timestamps from a set of in and out times for a specific id.
fn calculate_data_timestamps(
    in_times: &[InLine], 
    out_times: &[OutLine], 
    missing: &Missing,
    day_end: &NaiveDateTime,
) -> Vec<DataTimestamp> {
    let mut data_timestamps = Vec::<DataTimestamp>::new();
    let last_in_time = in_times
        .iter()
        .map(|line| line.time)
        .max().unwrap_or(NaiveDateTime::MIN);
    let last_out_time = out_times
        .iter()
        .map(|line| line.time)
        .max()
        .unwrap_or(NaiveDateTime::MAX);
    if last_in_time > last_out_time {
        match missing {
            Missing::Yes => data_timestamps.push(
                DataTimestamp { 
                    timestamp: Timestamp { 
                        start: last_in_time,
                        end: *day_end 
                    }, 
                    is_in: false, 
                    parked: None, 
                    people_in_car: None, 
                    activities: Vec::new()
                }
            ),
            Missing::No(end_time) => data_timestamps.push(
                DataTimestamp { 
                    timestamp: Timestamp { 
                        start: *end_time, 
                        end: *day_end,
                    }, 
                    is_in: false, 
                    parked: None, 
                    people_in_car: None,
                    activities: Vec::new(),
                }
            ),
        }
    }
    for in_time in in_times {
        let timestamp = Timestamp { 
            start: in_time.time,
            end: in_time.time,
        };
        data_timestamps.push(
            DataTimestamp { 
                timestamp, 
                is_in: true, 
                parked: None, 
                people_in_car: None,
                activities: Vec::new(),
            }
        )
    }
    for out_time in out_times {
        let timestamp = Timestamp { 
            start: out_time.time, 
            end: out_time.time,
        };
        data_timestamps.push(
            DataTimestamp { 
                timestamp, 
                is_in: false, 
                parked: Some(out_time.parked), 
                people_in_car: Some(out_time.people_in_car),
                activities: out_time.activities
                    .split(",")
                    .filter(|s| !s.is_empty())
                    .map(|s| FromStr::from_str(s.trim())
                        .expect("Invalid activity found"))
                    .collect::<Vec<Activity>>(),
            }
        )
    }
    data_timestamps
}


/// Finds all visits from set of in and out times for a specific id.
/// Panics if given invalid data
fn calculate_visits(
    data_timestamps: &[DataTimestamp], 
    weather: Weather
) -> Vec<Visit> {
    let mut data_timestamps: Vec<DataTimestamp> = data_timestamps.to_vec();
    data_timestamps.sort_by_key(|t| t.timestamp);

    let mut visits = Vec::<Visit>::new();
    let mut in_data_timestamp: Option<DataTimestamp> = None;
    for data_timestamp in data_timestamps {
        if data_timestamp.is_in {
            in_data_timestamp = Some(data_timestamp);
        } else {
            let Some(in_data_timestamp) = in_data_timestamp.take() 
                else { continue; };
            let out_data_timestamp = data_timestamp;
            let in_time = if in_data_timestamp.timestamp.is_exact() {
                in_data_timestamp.timestamp.start
            } else { continue; };
            let out_time = if out_data_timestamp.timestamp.is_exact() {
                out_data_timestamp.timestamp.end
            } else { continue; };
            let parked = match out_data_timestamp.parked {
                Some(parked) => parked,
                None => panic!("{} has no record of parking status", 
                    out_data_timestamp.timestamp.start),
            };
            let people_in_car = match out_data_timestamp.people_in_car {
                Some(people_in_car) => people_in_car,
                None => panic!("{} has no record of people in car", 
                    out_data_timestamp.timestamp.start),
            };
            let activities = out_data_timestamp.activities;
            visits.push(Visit { 
                in_time, 
                out_time, 
                parked, 
                people_in_car, 
                activities,
                weather,
            });
        }
    }
    visits
}


/// Finds all censoredly observed visits from set of exact in times 
/// and inexact out times for a specific id.
fn calculate_censored_visits(
    data_timestamps: &[DataTimestamp]
) -> Vec<CensoredVisit> {
    let mut data_timestamps: Vec<DataTimestamp> = data_timestamps.to_vec();
    data_timestamps.sort_by_key(|t| t.timestamp);

    let mut visits = Vec::<CensoredVisit>::new();
    let mut in_data_timestamp: Option<DataTimestamp> = None;
    for data_timestamp in data_timestamps {
        if data_timestamp.is_in {
            in_data_timestamp = Some(data_timestamp);
        } else {
            let Some(in_data_timestamp) = in_data_timestamp.take() 
                else { continue; };
            let out_data_timestamp = data_timestamp;
            let in_time = if in_data_timestamp.timestamp.is_exact() {
                in_data_timestamp.timestamp.start
            } else { continue; };
            // assume tagged cars in the lot at the end of observation window are parked
            // we can assume this because we stop tagging cars an hour before our end time
            let parked = match out_data_timestamp.parked {
                Some(parked) => parked,
                None => true,
            };
            visits.push(CensoredVisit { 
                parked,
                in_time, 
                out_time_start: out_data_timestamp.timestamp.start,
                out_time_end: out_data_timestamp.timestamp.end,
            });
        }
    }
    visits
}


/// loads all 
pub fn load_day(date: &str) -> 
    (Vec<Visit>, Vec<CensoredVisit>, Vec<Timestamp>, Vec<Timestamp>, ResearchTime) 
{
    let in_path = "input_data/in_data/".to_string() + date + " - In.csv";
    let in_data = read_in_file(&in_path); 
    let mut in_times: [Vec<InLine>; TOTAL_NUMBER_OF_CARDS] = 
        array::from_fn(|_| Vec::new());
    for line in in_data { in_times[line.code - 1].push(line); }
    
    let out_path = "input_data/out_data/".to_string() + date + " - Out.csv";
    let out_data = read_out_file(&out_path);
    let mut out_times: [Vec<OutLine>; TOTAL_NUMBER_OF_CARDS] = 
        array::from_fn(|_| Vec::new());
    for line in out_data { out_times[line.code - 1].push(line); }

    let missing_path = 
        "input_data/missing_card_data/".to_string()
        + date 
        + " - Missing Cards.csv";
    let missing_data = read_missing_cards_file(&missing_path);

    let card_distribution_time_path = 
        "input_data/card_distribution_time_data/".to_string() 
        + date 
        + " - Card Distribution Time.csv";
    let card_distribution_time = 
        read_card_distribution_time_file(&card_distribution_time_path);

    let weather_path = 
        "input_data/weather_data/".to_string()
        + date
        + " - Weather.txt";
    let weather = Weather::from_str(
        &fs::read_to_string(weather_path).unwrap().trim()
    ).unwrap();

    let research_time = ResearchTime { 
        start: card_distribution_time.in_start_time,
        end: card_distribution_time.out_end_time,
        weather,
    };

    let day_end = NaiveDateTime::parse_from_str(
        &format!("2026-{} 22:00:00", date), "%Y-%m-%d %H:%M:%S"
    ).expect("Failed to parse datetime");
     
    let mut visits = Vec::<Visit>::new();
    let mut censored_visits = Vec::<CensoredVisit>::new();
    let mut in_timestamps = Vec::<Timestamp>::new();
    let mut out_timestamps = Vec::<Timestamp>::new();
    for id in 0..TOTAL_NUMBER_OF_CARDS {
        let missing = if missing_data.contains(&(id + 1)) { Missing::Yes } 
                      else { Missing::No(card_distribution_time.out_end_time) };
        let data_timestamps = calculate_data_timestamps(
            &in_times[id], 
            &out_times[id], 
            &missing,
            &day_end,
        );
        visits.extend(calculate_visits(&data_timestamps, weather));
        censored_visits.extend(calculate_censored_visits(&data_timestamps));
        let (ins, outs): (Vec<DataTimestamp>, Vec<DataTimestamp>) = data_timestamps
            .into_iter()
            .partition(|t| t.is_in);
        in_timestamps.extend(ins.into_iter().map(Timestamp::from));
        out_timestamps.extend(outs.into_iter().map(Timestamp::from));
    }

    (visits, censored_visits, in_timestamps, out_timestamps, research_time)
}

