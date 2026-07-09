use data_merge::input::load_dates;
use data_merge::output::{ 
    output_research_times, 
    output_timestamps, 
    output_visits,
    output_censored_visits,
};

pub const NUMBER_OF_DATES: usize = 14;
pub const DATES: [&str; NUMBER_OF_DATES] = [
    "6-17",
    "6-18",
    "6-19",
    "6-20",
    "6-22",
    "6-23",
    "6-25",
    "6-26",
    "6-27",
    "6-28",
    "6-29",
    "6-30",
    "7-2",
    "7-8",
];

fn main() {
    let (
        visits, 
        censored_visits, 
        in_timestamps, 
        out_timestamps, 
        research_times
    ) = load_dates(&DATES);
    output_visits(&visits);
    output_censored_visits(&censored_visits);
    output_timestamps(&in_timestamps, "entrance");
    output_timestamps(&out_timestamps, "exit");
    output_research_times(&research_times);

    let visit_count = visits.len();
    let censored_visit_count = censored_visits.len();
    let in_timestamp_count = in_timestamps.len();
    let out_timestamp_count = out_timestamps.len();
    println!("{visit_count} complete visits");
    println!("{censored_visit_count} censored visits");
    println!("{in_timestamp_count} logged entrances");
    println!("{out_timestamp_count} logged exits"); 
}

