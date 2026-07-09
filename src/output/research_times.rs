use crate::core::{ ResearchTime, Weather };
use serde::Serialize;
use chrono::{ NaiveTime, Weekday, Datelike };

#[derive(Serialize)]
struct TimeLine {
    time: NaiveTime,
    total: u8,
    
    sunny: u8,
    rain: u8,
    on_off_rain: u8,
    cloudy: u8,
    thunderstorm: u8,

    monday: u8,
    tuesday: u8,
    wednesday: u8,
    thursday: u8,
    friday: u8,
    saturday: u8,
    sunday: u8,
}

/// Writes visit data to "visits.csv"
pub fn output_research_times(research_times: &Vec<ResearchTime>) {
    let times: [TimeLine; 901] = std::array::from_fn(|i| {
        let m: u32 = i as u32 + 360;
        let minute = NaiveTime::from_hms_opt(m / 60, m % 60, 0)
            .expect("Valid time generation");
        let total_research_times: Vec<&ResearchTime> = research_times
            .iter()
            .filter(|t| t.is_within(minute))
            .collect();
        TimeLine {
            time: minute,
            total: total_research_times.len() as u8,
            sunny: total_research_times
                .iter()
                .filter(|t| t.weather == Weather::Sunny)
                .count() as u8,
            rain: total_research_times
                .iter()
                .filter(|t| t.weather == Weather::Rain)
                .count() as u8,
            on_off_rain: total_research_times
                .iter()
                .filter(|t| t.weather == Weather::OnOffRain)
                .count() as u8,
            cloudy: total_research_times
                .iter()
                .filter(|t| t.weather == Weather::Cloudy)
                .count() as u8,
            thunderstorm: total_research_times
                .iter()
                .filter(|t| t.weather == Weather::Thunderstorm)
                .count() as u8,
            monday: total_research_times
                .iter()
                .filter(|t| t.start.weekday() == Weekday::Mon)
                .count() as u8,
            tuesday: total_research_times
                .iter()
                .filter(|t| t.start.weekday() == Weekday::Tue)
                .count() as u8,
            wednesday: total_research_times
                .iter()
                .filter(|t| t.start.weekday() == Weekday::Wed)
                .count() as u8,
            thursday: total_research_times
                .iter()
                .filter(|t| t.start.weekday() == Weekday::Thu)
                .count() as u8,
            friday: total_research_times
                .iter()
                .filter(|t| t.start.weekday() == Weekday::Fri)
                .count() as u8,
            saturday: total_research_times
                .iter()
                .filter(|t| t.start.weekday() == Weekday::Sat)
                .count() as u8,
            sunday: total_research_times
                .iter()
                .filter(|t| t.start.weekday() == Weekday::Sun)
                .count() as u8,
        }
    });

    let mut wtr = csv::Writer::from_path(
        "output_data/research_times.csv"
    ).unwrap();
    for time in &times { 
        wtr.serialize(time).unwrap(); 
    }
    wtr.flush().unwrap();
    let mut wtr = csv::Writer::from_path(
        "../merged_data/research_times.csv"
    ).unwrap();
    for time in &times { 
        wtr.serialize(time).unwrap(); 
    }
    wtr.flush().unwrap();
}
