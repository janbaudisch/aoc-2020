mod password;

use common::input;
use password::Password;

fn main() {
    let passwords: Vec<Password> = input::read_lines()
        .iter()
        .map(|x| x.to_string().into())
        .collect();
    
    let correct: u32 = passwords.clone().iter().map(|x| {
        if x.check_sled_rental() {
            return 1;
        } else {
            return 0;
        }
    }).sum();

    println!(
        "[PART ONE] number of correct passswords: {}",
        correct
    );

    let correct: u32 = passwords.clone().iter().map(|x| {
        if x.check_toboggan_corporate() {
            return 1;
        } else {
            return 0;
        }
    }).sum();

    println!(
        "[PART TWO] number of correct passswords: {}",
        correct
    );
}

#[cfg(test)]
mod tests {
    use super::Password;

    #[test]
    fn part_one() {
        assert!(Password::from("1-3 a: abcde".to_string()).check_sled_rental());
        assert!(!Password::from("1-3 b: cdefg".to_string()).check_sled_rental());
        assert!(Password::from("2-9 c: ccccccccc".to_string()).check_sled_rental());
    }

    #[test]
    fn part_two() {
        assert!(Password::from("1-3 a: abcde".to_string()).check_toboggan_corporate());
        assert!(!Password::from("1-3 b: cdefg".to_string()).check_toboggan_corporate());
        assert!(!Password::from("2-9 c: ccccccccc".to_string()).check_toboggan_corporate());
    }
}
