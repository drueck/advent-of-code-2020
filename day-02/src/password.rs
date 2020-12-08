use regex::Regex;

pub struct PasswordEntry {
    min: u8,
    max: u8,
    letter: char,
    password: String,
}

impl PasswordEntry {
    pub fn from_string(s: String) -> PasswordEntry {
        let re = Regex::new(r"(\d+)-(\d+) (.): (.+)").unwrap();
        let captures = re.captures(&s).unwrap();

        let min = captures.get(1).unwrap().as_str().parse::<u8>().unwrap();
        let max = captures.get(2).unwrap().as_str().parse::<u8>().unwrap();
        let letter = captures.get(3).unwrap().as_str().chars().next().unwrap();
        let password = captures.get(4).unwrap().as_str().to_string();

        return PasswordEntry {
            min,
            max,
            letter,
            password,
        };
    }

    pub fn valid(&self) -> bool {
        let character_count = self.password.matches(self.letter).count() as u8;
        return character_count >= self.min && character_count <= self.max;
    }
}

#[cfg(test)]
mod tests {
    use crate::password::PasswordEntry;

    #[test]
    fn test_from_string() {
        let entry = PasswordEntry::from_string("7-15 y: asldfjlaksjdflkjasdy".to_string());

        assert_eq!(entry.min, 7u8);
        assert_eq!(entry.max, 15u8);
        assert_eq!(entry.letter, 'y');
        assert_eq!(entry.password, "asldfjlaksjdflkjasdy");
    }

    #[test]
    fn test_valid() {
        let too_few = PasswordEntry::from_string("20-30 x: aslkxxjalkx".to_string());
        let too_many = PasswordEntry::from_string("1-2 y: asdyyasdy".to_string());
        let just_right = PasswordEntry::from_string("3-5 z: 123xyzzzkz".to_string());

        assert_eq!(too_few.valid(), false);
        assert_eq!(too_many.valid(), false);
        assert_eq!(just_right.valid(), true);
    }
}
