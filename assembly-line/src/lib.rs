// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

fn success_rate(speed: u8) -> f64 {
    return match speed{
        0..=4 => Some(1.0),
        5..=8 => Some(0.9),
        9..=10 => Some(0.77),
        11..=u8::MAX => None
    }.expect("Speed must be between 1 and 10 inclusive.");
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let r = success_rate(speed);
    let total_production = speed as u32 * 221;

    return total_production as f64 * r
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    return (production_rate_per_hour(speed) / 60.0) as u32
}
