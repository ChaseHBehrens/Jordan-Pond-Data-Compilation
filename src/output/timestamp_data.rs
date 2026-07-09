use crate::core::Timestamp;


/// Writes visit data to "visits.csv"
pub fn output_timestamps(timestamps: &[Timestamp], name: &str) {
    let mut wtr = csv::Writer::from_path(
        "output_data/".to_string() + name + ".csv"
    ).unwrap();
    for timestamp in timestamps { 
        wtr.serialize(timestamp).unwrap(); 
    }
    wtr.flush().unwrap();
    let mut wtr = csv::Writer::from_path(
        "../merged_data/".to_string() + name + ".csv"
    ).unwrap();
    for timestamp in timestamps { 
        wtr.serialize(timestamp).unwrap(); 
    }
    wtr.flush().unwrap();
}

