use std::{fmt, string};

#[derive(Debug, Clone)]
pub struct SummationError {
    msg: String,
}

impl std::error::Error for SummationError {}
impl fmt::Display for SummationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}
impl From<std::io::Error> for SummationError {
    fn from(e: std::io::Error) -> SummationError {
        SummationError {
            msg: format!("io::Error: {}", e),
        }
    }
}
impl From<std::num::ParseIntError> for SummationError {
    fn from(e: std::num::ParseIntError) -> SummationError {
        SummationError {
            msg: format!("ParseIntError: {}", e),
        }
    }
}

pub fn sum_file_1(file: &std::path::Path) -> Result<i64, SummationError> {
    let mut sum = 0;
    let file_as_string = std::fs::read_to_string(file);

    let check_proper_read = match file_as_string {
        Ok(file_str) => file_str,
        Err(e) => return Err(SummationError { msg: e.to_string() }),
    };

    let file_trimmed_string = check_proper_read.trim().split('\n');

    for line in file_trimmed_string.into_iter() {
        let parsed_int = line.parse::<i64>();
        let parse_success = match parsed_int {
            Ok(int) => sum += parsed_int.unwrap(),
            Err(e) => return Err(SummationError::from(e)),
        };
    }

    return Ok(sum);
}

pub fn sum_file_2(file: &std::path::Path) -> Result<i64, SummationError> {
    let mut sum = 0;
    let file_as_string = std::fs::read_to_string(file)?;
    let file_trimmed_string = file_as_string.trim().split('\n');

    for line in file_trimmed_string.into_iter() {
        let parsed_int = line.parse::<i64>()?;
        sum += parsed_int
    }

    return Ok(sum);
}
