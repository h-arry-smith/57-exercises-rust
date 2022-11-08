use std::fmt::Display;

enum PasswordStrength {
    VeryWeak,
    Weak,
    Average,
    Strong,
    VeryStrong,
}

struct Password {
    text: String,
}

impl Password {
    fn new(text: &str) -> Self {
        Password {
            text: text.to_owned(),
        }
    }

    fn strength(&self) -> PasswordStrength {
        // If a password is less than eight characters
        if self.text.len() < 8 {
            // If it only consists of numbers, then it is very weak
            if self.only_numbers() {
                return PasswordStrength::VeryWeak;
            }
            // Otherwise, it is weak
            else {
                return PasswordStrength::Weak;
            }
        }
        // If a password is longer than eight characters
        else {
            // If it is complex (contains numbers, letters, and special characters), then it is very strong
            if self.is_complex() {
                return PasswordStrength::VeryStrong;
            }
            // Otherwise, if it contains at least one number it is strong
            if self.numbers_present() >= 1 {
                return PasswordStrength::Strong;
            }

            // Otherwise, it is of average strength
            PasswordStrength::Average
        }
    }

    fn only_numbers(&self) -> bool {
        match self.text.parse::<usize>() {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn is_complex(&self) -> bool {
        self.numbers_present() >= 1 && self.special_characters_present() >= 1
    }

    fn numbers_present(&self) -> usize {
        let mut count = 0;

        self.text.chars().for_each(|item| {
            if item.is_numeric() {
                count += 1;
            }
        });

        count
    }

    fn special_characters_present(&self) -> usize {
        let special_characters = ['#', '@', '!', '£', '$', '%', '^', '&', '*'];
        let mut count = 0;

        self.text.chars().for_each(|item| {
            if special_characters.contains(&item) {
                count += 1;
            }
        });

        count
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl Display for PasswordStrength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PasswordStrength::VeryWeak => "very week",
                PasswordStrength::Weak => "weak",
                PasswordStrength::Average => "average",
                PasswordStrength::Strong => "strong",
                PasswordStrength::VeryStrong => "very strong",
            }
        )
    }
}

fn main() {
    let passwords = ["12345", "abcdef", "abcdefghijk", "abc123xyz", "1337h@xor!"];

    for password in passwords {
        let password = Password::new(password);
        let strength = password.strength();

        println!("The password '{password}' is a {strength} password.");
    }
}
