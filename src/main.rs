mod bmi;
mod extensions;

use bmi::BMI;
use extensions::{FloatExtensions, StringExtensions};
use std::io::stdin;
use std::process::exit;
use colored::*;

fn get_user_input(question: &str) -> String {
    println!("{}", question.bright_yellow());

    let mut resp = String::new();
    stdin()
        .read_line(&mut resp)
        .expect("trying to get user input");

    resp.trim().to_string()
}

fn main() {
    println!("BMI calculator!");

    let height = get_user_input("Height? (inches)").parse_string_to_f32();

    println!("{}", format!("\nYour height in feet: {:.2} ft", height.inchs_to_foot()).italic());
    println!("{}", format!("Your height in cms: {:.2} cms\n", height.inches_to_cms()).italic());

    let weight = get_user_input("Weight? (kilograms)").parse_string_to_f32();

    println!("{}", format!("\nYour weight in lbs: {:.2} lbs\n", weight.kgs_to_lbs()).italic());

    let bmi = match BMI::new(height, weight) {
        Ok(res) => res,
        Err(err) => {
            println!("{}", err);
            exit(1);
        }
    };

    bmi.print();
}
