use std::{ cmp::Ordering, str::FromStr, fmt };
use chrono::{ NaiveDateTime, NaiveTime };
use serde::Serialize;

/// A time log of a QR code.
#[derive(Eq, Clone, Copy, Debug, Serialize)]
pub struct Timestamp {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}

impl From<DataTimestamp> for Timestamp {
    fn from(data_timestamp: DataTimestamp) -> Self {
        data_timestamp.timestamp
    }
}

impl Timestamp {
    fn to_sort_key(&self) -> NaiveDateTime { self.start }
    pub fn is_exact(&self) -> bool { self.start == self.end }
}

impl PartialEq for Timestamp {
    fn eq(&self, other: &Self) -> bool {
        self.to_sort_key() == other.to_sort_key()
    }
}

impl PartialOrd for Timestamp {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Timestamp {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_sort_key().cmp(&other.to_sort_key())
    }
}


/// represents a timestamp with associated data
#[derive(Clone)]
pub struct DataTimestamp {
    pub timestamp: Timestamp,
    pub is_in: bool,
    pub parked: Option<bool>,
    pub people_in_car: Option<u8>,
    pub activities: Vec<Activity>,
}


/// Represents what people do at Jordan Pond
#[derive(Clone, PartialEq)]
pub enum Activity {
    JordanPond,
    JordanPondHouse,
    Trails,
}

impl FromStr for Activity {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Jordan Pond House" => Ok(Self::JordanPondHouse), 
            "Other Walking Trails" => Ok(Self::Trails), 
            "Jordan Pond" => Ok(Self::JordanPond),
            _ => Err(format!("'{}' is not a valid Activity", s)),
        }
    }
}


/// Represents the weather at Jordan Pond
#[derive(Copy, Clone, PartialEq)]
pub enum Weather {
    Sunny,
    Rain,
    OnOffRain,
    Cloudy,
    Thunderstorm,
}

impl FromStr for Weather {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Sunny" => Ok(Self::Sunny), 
            "Rain" => Ok(Self::Rain), 
            "On and Off Rain" => Ok(Self::OnOffRain),
            "Cloudy" => Ok(Self::Cloudy),
            "Thunderstorm" => Ok(Self::Thunderstorm),
            _ => Err(format!("'{}' is not a valid weather", s)),
        }
    }
}

impl fmt::Display for Weather {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let weather_str = match self {
            Self::Sunny => "Sunny",
            Self::Rain => "Rain",
            Self::OnOffRain => "On and Off Rain",
            Self::Cloudy => "Cloudy",
            Self::Thunderstorm => "Thunderstorm",
        };
        write!(f, "{}", weather_str)
    }
}


/// Represents data collected from a visit
pub struct Visit {
    pub in_time: NaiveDateTime,
    pub out_time: NaiveDateTime,
    pub parked: bool,
    pub people_in_car: u8,
    pub activities: Vec<Activity>,
    pub weather: Weather,
}

impl Visit {
    /// Returns the duration of the visit in minutes
    pub fn duration(&self) -> usize {
        let duration = self.out_time - self.in_time;
        duration.num_minutes() as usize
    }
}


/// Represents data collected from a fully observed visit
/// or a visit that extended beyond the observation window
pub struct CensoredVisit {
    pub parked: bool,
    pub in_time: NaiveDateTime,
    pub out_time_start: NaiveDateTime,
    pub out_time_end: NaiveDateTime,
}

impl CensoredVisit {
    /// Returns the minimum possible duration of the visit in minutes
    pub fn min_duration(&self) -> usize {
        let duration = self.out_time_start - self.in_time;
        duration.num_minutes() as usize
    }
    
    /// Returns the maximum possible duration of the visit in minutes
    pub fn max_duration(&self) -> usize {
        let duration = self.out_time_end - self.in_time;
        duration.num_minutes() as usize
    }
}


/// Represents a time period of research
pub struct ResearchTime {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub weather: Weather,
}

impl ResearchTime {
    pub fn is_within(&self, time: NaiveTime) -> bool {
        time >= self.start.time() && time <= self.end.time()
    }
}

