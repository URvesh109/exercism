const LASAGNA_OVEN_TIME: u32 = 40;
const LASAGNA_LAYER_TIME:u32 = 2;

fn expected_minutes_in_oven() -> u32{
    LASAGNA_OVEN_TIME
}

fn remaining_minutes_oven(time: u32) -> u32 {
    LASAGNA_OVEN_TIME - time
}

fn preparation_time_in_minutes(layer: u32) ->u32 {
    LASAGNA_LAYER_TIME * layer
}

fn elapsed_time_in_minutes(layer:u32, oven_time: u32) -> u32 {
    preparation_time_in_minutes(layer) + oven_time

}

fn main() {
    println!("expected_minutes_in_oven {}", expected_minutes_in_oven());
    println!("remaining_minutes_oven {}", remaining_minutes_oven(30));
    println!("preparation_time_in_minutes {}", preparation_time_in_minutes(2));
    println!("elapsed_time_in_minutes {}", elapsed_time_in_minutes(3, 20))
}
