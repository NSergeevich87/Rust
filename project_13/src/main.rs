mod print;
use regex::Regex;
fn main() {
    print::print_name();

    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Is 2021-01-01 a date? {}", re.is_match("2021-01-01"));

    car_factory::build_car();
}

mod car_factory {
    pub fn build_car() {
        println!("Honk honk!");
    }
}