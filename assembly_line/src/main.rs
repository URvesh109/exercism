const CAR_PER_HOUR: u32 = 221;

fn production_rater_hour(rate: u32) -> f64{
    match rate {
        1..=4 => (CAR_PER_HOUR * rate) as f64 * 1.0,
        5..=8 => (CAR_PER_HOUR * rate) as f64 * 0.9,
        9..=10 => (CAR_PER_HOUR * rate) as f64 * 7.7,
        _ => 0.0
    }
}

fn working_item_per_minute(rate: u32) -> u32 {
    production_rater_hour(rate) as u32 / 60
}


fn main() {
    println!("working item {}", working_item_per_minute(6));
}
