use super::extensions::FloatExtensions;
use std::fmt::Display;
use colored::*;

#[derive(Debug)]
pub struct BMI {
    weight: f32,
    height: f32,
    bmi: f32,
}

impl BMI {
    pub fn new(height: f32, weight: f32) -> Result<Self, BMIError> {
        if height <= 0f32 || height > 120f32 {
            return Err(BMIError::InvalidHeight(String::from("Invalid height!")));
        }

        if weight <= 0f32 || weight > 500f32 {
            return Err(BMIError::InvalidWeight(String::from("Invalid weight!")));
        }

        let bmi = (weight.kgs_to_lbs() * 703f32) / (height * height);

        Ok(Self {
            height,
            weight,
            bmi,
        })
    }

    pub fn get_bmi(&self) -> f32 {
        self.bmi
    }

    pub fn get_range(&self) -> BMIRange {
        if self.bmi < 18.5 {
            BMIRange::UnderWeight
        } else if self.bmi >= 18.5 && self.bmi <= 24.9 {
            BMIRange::Healthy
        } else if self.bmi >= 25.0 && self.bmi <= 29.9 {
            BMIRange::OverWeight
        } else {
            BMIRange::Obese
        }
    }

    pub fn get_range_colored(&self) -> ColoredString {
        let range = self.get_range();
        let range_as_string = format!("{:?}", range);

        match range {
            BMIRange::Healthy => range_as_string.bright_green(),
            BMIRange::UnderWeight => range_as_string.bright_blue(),
            BMIRange::OverWeight => range_as_string.bright_magenta(),
            BMIRange::Obese => range_as_string.bright_red(),
        }
    }

    pub fn print(&self) {
        println!("{}: {:.2} ({})", "BMI".bright_yellow(),  self.get_bmi(), self.get_range_colored())
    }
}

#[derive(Debug)]
pub enum BMIRange {
    UnderWeight,
    Healthy,
    OverWeight,
    Obese,
}

#[derive(Debug)]
pub enum BMIError {
    InvalidHeight(String),
    InvalidWeight(String),
}

impl Display for BMIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_message = match &self {
            BMIError::InvalidHeight(err) => err,
            BMIError::InvalidWeight(err) => err,
        };
        write!(f, "BMI error: {}", error_message)
    }
}
