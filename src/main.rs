#[macro_use]
extern crate clap;

use clap::App;

mod binary_boarding;
mod custom_customs;
mod handheld_halting;
mod handy_haversacks;
mod passport_processing;
mod password_philosophy;
mod report_repair;
mod toboggan_trajectory;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    let input = matches.value_of("input").unwrap();
    
    match matches.value_of("program").unwrap() {
        "binary"   => binary_boarding::run(input),
        "handheld" => handheld_halting::run(input),
        "handy"    => handy_haversacks::run(input),
        "custom"   => custom_customs::run(input),
        "report"   => report_repair::run(input),
        "password" => password_philosophy::run(input),
        "toboggan" => toboggan_trajectory::run(input),
        "passport" => passport_processing::run(input),
        _ => println!("Bad program number!"),
    }
}
