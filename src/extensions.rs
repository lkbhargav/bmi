pub trait StringExtensions {
    fn parse_string_to_f32(&self) -> f32;
}

impl StringExtensions for String {
    fn parse_string_to_f32(&self) -> f32 {
        match self.trim().parse() {
            Ok(v) => v,
            Err(e) => {
                println!(
                    "Something went wrong trying to parse {} to f32. Error: {}",
                    self, e
                );
                0f32
            }
        }
    }
}

pub trait FloatExtensions {
    fn kgs_to_lbs(&self) -> f32;
    fn lbs_to_kgs(&self) -> f32;
    fn inchs_to_foot(&self) -> f32;
    fn inches_to_cms(&self) -> f32;
}

const ONE_POUND: f32 = 0.45359237;
const ONE_CENTIMETER: f32 = 2.54;

impl FloatExtensions for f32 {
    fn kgs_to_lbs(&self) -> f32 {
        self / ONE_POUND
    }

    fn lbs_to_kgs(&self) -> f32 {
        self * ONE_POUND
    }

    fn inchs_to_foot(&self) -> f32 {
        self / 12f32
    }

    fn inches_to_cms(&self) -> f32 {
        self * ONE_CENTIMETER
    }
}
