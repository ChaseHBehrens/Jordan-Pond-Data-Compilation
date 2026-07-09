mod card_data;
mod clicker_data;
mod parsers;

use crate::core::{ ResearchTime, Timestamp, Visit, CensoredVisit };

pub fn load_dates(dates: &[&str]) -> (
    Vec<Visit>, 
    Vec<CensoredVisit>,
    Vec<Timestamp>, 
    Vec<Timestamp>,
    Vec<ResearchTime>,
) {
    let mut visits = Vec::<Visit>::new();
    let mut censored_visits = Vec::<CensoredVisit>::new();
    let mut in_timestamps = Vec::<Timestamp>::new();
    let mut out_timestamps = Vec::<Timestamp>::new();
    let mut research_times = Vec::<ResearchTime>::new();
    for date in dates {
        let (
            card_visits, 
            card_censored_visits,
            card_in_timestamps, 
            card_out_timestamps, 
            research_time,
        ) = card_data::load_day(date);
        visits.extend(card_visits);
        censored_visits.extend(card_censored_visits);
        in_timestamps.extend(card_in_timestamps);
        out_timestamps.extend(card_out_timestamps);
        research_times.push(research_time);
        
        let (clicker_in_timestamps, clicker_out_timestamps) = 
            clicker_data::load_day(date);
        in_timestamps.extend(clicker_in_timestamps);
        out_timestamps.extend(clicker_out_timestamps);
    }
    (visits, censored_visits, in_timestamps, out_timestamps, research_times)
}
