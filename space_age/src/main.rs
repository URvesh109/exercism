
struct Duration(f64);

impl From<u64> for Duration {
    fn from(sec: u64) -> Self {
        Duration (sec as f64)
    }
}

const EARTH: f64 = 31557600.0;

trait Planet {
    const ORBITAL_PERIOD: f64;
    fn calculate_years(d: &Duration) -> f64 {
        (d.0 / EARTH) / Self::ORBITAL_PERIOD
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;

impl Planet for Mercury {
    const ORBITAL_PERIOD:f64 = 0.24084;
}

impl Planet for Venus {
    const ORBITAL_PERIOD:f64 = 0.61519726;
}

impl Planet for Earth {
    const ORBITAL_PERIOD: f64 = 1.0;
}

fn main() {
    let duration = Duration::from(1_000_000_000);
    let earth = Earth::calculate_years(&duration);
    println!("{earth}");
}