use time::PrimitiveDateTime as DateTime;
use time::ext::NumericalDuration;
use time::{Date, Time, Month};

fn after(start: DateTime) -> DateTime {
    start + 1e9.seconds()
}

fn main() {
    let date = Date::from_calendar_date(2022, Month::February, 22);
    let time = Time::from_hms_milli(10, 00, 00, 00);
    let date_time = DateTime::new(date.unwrap(), time.unwrap());
    println!("{:?}", after(date_time));
}
