extern crate reqwest;

mod bmi;
mod extensions;

use bmi::BMI;
use extensions::{FloatExtensions, StringExtensions};
use reqwest::blocking::ClientBuilder;
use reqwest::redirect;
use reqwest::Url;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::stdin;
use std::process::exit;
use std::sync::{Arc, Mutex};

fn get_user_input(question: &str) -> String {
    println!("{}", question);

    let mut resp = String::new();
    stdin()
        .read_line(&mut resp)
        .expect("trying to get user input");

    resp.trim().to_string()
}

fn main() {
    println!("BMI calculator!");

    let height = get_user_input("Height? (inches)").parse_string_to_f32();

    println!("Your height in feet: {:.2} feet", height.inchs_to_foot());
    println!("Your height in cms: {:.2} cms\n", height.inches_to_cms());

    let weight = get_user_input("Weight? (kilograms)").parse_string_to_f32();

    println!("Your weight in lbs: {:.2} lbs\n", weight.kgs_to_lbs());

    let bmi = match BMI::new(height, weight) {
        Ok(res) => res,
        Err(err) => {
            println!("{}", err);
            exit(1);
        }
    };

    println!(
        "Your BMI is {:.2}! And you are {:?}!",
        bmi.get_bmi(),
        bmi.get_range()
    );

    // let resp = reqwest::blocking::get("https://httpbin.org/ip").unwrap()
    //     .json::<HashMap<String, String>>().unwrap();
    // println!("{:#?}", resp);

    // let mut status_codes = Vec::<u16>::new();
    // let mut redirect_urls = Vec::<String>::new();

    // let client = reqwest::blocking::Client::builder()
    // .redirect(reqwest::redirect::Policy::custom( |attempt| {
    //     let next_url = attempt.url().to_string();
    //     let status_code = attempt.status();
    //     println!("Next URL: {} | Status code: {}", next_url, status_code);
    //     attempt.follow()
    // })).build().unwrap();

    // let resp = client.get("https://go.bluegreenanalytics.com").send().unwrap();
}
