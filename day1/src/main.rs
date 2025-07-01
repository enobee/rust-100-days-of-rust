fn calc_age(age: u32) -> u32 {
    age * 365
}

fn main() {
    let age_in_days = calc_age(0);
    println!("Age in days {}", age_in_days);
}
