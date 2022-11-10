use std::fmt::Debug;
use std::io;
use std::io::Write;

enum ValidateResult<T> {
    Valid(T),
    Invalid(String),
}

trait Valid<T> {
    fn new(value: T) -> Self
    where
        Self: Sized;

    fn try_new(value: T) -> Result<Self, ValidationError>
    where
        Self: Sized,
        T: ToOwned,
    {
        match Self::do_validation(value) {
            ValidateResult::Valid(value) => Ok(Self::new(value)),
            ValidateResult::Invalid(err) => Err(ValidationError(err)),
        }
    }

    fn do_validation(value: T) -> ValidateResult<T>
    where
        Self: Sized,
    {
        if Self::validate(&value) {
            ValidateResult::Valid(value)
        } else {
            ValidateResult::Invalid(Self::failure_reason(&value))
        }
    }

    fn validate(value: &T) -> bool
    where
        Self: Sized;
    fn failure_reason(value: &T) -> String
    where
        Self: Sized;
}

#[derive(Debug)]
struct ValidationError(String);

#[derive(Debug)]
struct ValidEmployeeName {
    value: String,
}

impl Valid<&str> for ValidEmployeeName {
    fn new(value: &str) -> Self {
        ValidEmployeeName {
            value: value.to_string(),
        }
    }

    fn validate(value: &&str) -> bool {
        value.len() > 2
    }

    fn failure_reason(value: &&str) -> String {
        format!("Employee name {value} is not valid. Must be longer than 1 charcter.").to_string()
    }
}

#[derive(Debug)]
struct ValidZipCode {
    value: String,
}

impl Valid<&str> for ValidZipCode {
    fn new(value: &str) -> Self {
        ValidZipCode {
            value: value.to_string(),
        }
    }

    fn validate(value: &&str) -> bool {
        value.len() == 5 && value.parse::<i32>().is_ok()
    }

    fn failure_reason(value: &&str) -> String {
        format!("The zip code {value} is not valid and must be 5 characters & numeric.").to_string()
    }
}

#[derive(Debug)]
struct ValidEmployeeID {
    value: String,
}

impl Valid<&str> for ValidEmployeeID {
    fn new(value: &str) -> Self {
        ValidEmployeeID {
            value: value.to_string(),
        }
    }

    fn validate(value: &&str) -> bool {
        let parts: Vec<&str> = value.split("-").collect();

        if parts.len() != 2 {
            return false;
        }

        let id_letters = parts[0];
        let id_numbers = parts[1];

        if id_letters.len() != 2 && !id_letters.chars().all(|c| c.is_alphabetic()) {
            return false;
        }

        if id_numbers.len() != 4 && !id_letters.chars().all(|c| c.is_numeric()) {
            return false;
        }

        true
    }

    fn failure_reason(value: &&str) -> String {
        format!("The employee ID {value} does not match the requried pattern of AA-1234.")
            .to_string()
    }
}

fn main() {
    let first_name = ValidEmployeeName::try_new(&get_string("Enter the first name: "));
    let last_name = ValidEmployeeName::try_new(&get_string("Enter the last name: "));
    let zipcode = ValidZipCode::try_new(&get_string("Enter the ZIP code: "));
    let id_tag = ValidEmployeeID::try_new(&get_string("Enter an employee ID: "));

    let mut error_count = 0;
    count_errors(&first_name, &mut error_count);
    count_errors(&last_name, &mut error_count);
    count_errors(&zipcode, &mut error_count);
    count_errors(&id_tag, &mut error_count);

    println!("There were {error_count} errors.");
}

fn count_errors<V: std::fmt::Debug>(value: &Result<V, ValidationError>, error_count: &mut usize) {
    if value.is_err() {
        *error_count += 1;
    }
}

fn get_string(question_text: &str) -> String {
    print!("{question_text}");
    io::stdout().flush().unwrap();

    let mut answer = String::new();

    io::stdin()
        .read_line(&mut answer)
        .expect("Could not read a line.");

    answer.trim().to_string()
}
