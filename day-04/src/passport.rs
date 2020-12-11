use regex::Regex;
use std::collections::HashMap;

type Validator = fn(String) -> bool;

pub struct Passport {
    fields: HashMap<String, String>,
    validators: HashMap<String, Validator>,
}

impl Passport {
    pub fn new(fields: &HashMap<String, String>) -> Passport {
        let mut validators: HashMap<String, Validator> = HashMap::new();

        validators.insert("byr".to_string(), validate_byr);
        validators.insert("iyr".to_string(), validate_iyr);
        validators.insert("eyr".to_string(), validate_eyr);
        validators.insert("hgt".to_string(), validate_hgt);
        validators.insert("hcl".to_string(), validate_hcl);
        validators.insert("ecl".to_string(), validate_ecl);
        validators.insert("pid".to_string(), validate_pid);

        return Passport {
            fields: fields.clone(),
            validators,
        };
    }

    pub fn valid(&self) -> bool {
        return self
            .validators
            .iter()
            .all(|(field, validate)| match self.fields.get(field) {
                None => false,
                Some(val) => validate(val.to_string()),
            });
    }
}

fn parse_year(val: String) -> Option<usize> {
    if val.len() != 4 {
        return None;
    }

    let parse_result = val.parse::<usize>();
    if parse_result.is_err() {
        return None;
    }
    let year = parse_result.unwrap();
    return Some(year);
}

fn validate_year_range(val: String, min: usize, max: usize) -> bool {
    match parse_year(val) {
        None => {
            return false;
        }
        Some(year) => {
            if year < min || year > max {
                return false;
            }
        }
    }
    return true;
}

fn validate_byr(val: String) -> bool {
    return validate_year_range(val, 1920, 2020);
}

fn validate_iyr(val: String) -> bool {
    return validate_year_range(val, 2010, 2020);
}

fn validate_eyr(val: String) -> bool {
    return validate_year_range(val, 2020, 2030);
}

fn validate_hgt(val: String) -> bool {
    let height_regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    return match height_regex.captures(&val) {
        None => false,
        Some(captures) => {
            let value = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let unit = captures.get(2).unwrap().as_str();
            match unit {
                "cm" => value >= 150 && value <= 293,
                "in" => value >= 59 && value <= 76,
                _ => false,
            }
        }
    };
}

fn validate_hcl(val: String) -> bool {
    let color_regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    return color_regex.is_match(&val);
}

fn validate_ecl(val: String) -> bool {
    return vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&val.as_str());
}

fn validate_pid(val: String) -> bool {
    let pid_regex = Regex::new(r"^[0-9]{9}$").unwrap();
    return pid_regex.is_match(&val);
}
