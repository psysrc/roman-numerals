pub fn roman_numerals(mut num: u32) -> String {
    if num == 0 {
        return String::from("Nulla");
    }

    let mut result = String::from("");

    while num >= 10 {
        result.push('X');
        num -= 10;
    }

    result.push_str(match num {
        0 => "",
        1 => "I",
        2 => "II",
        3 => "III",
        4 => "IV",
        5 => "V",
        6 => "VI",
        7 => "VII",
        8 => "VIII",
        9 => "IX",
        _ => panic!(),
    });

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(roman_numerals(0), "Nulla");
    }

    #[test]
    fn test_1_to_10() {
        assert_eq!(roman_numerals(1), "I");
        assert_eq!(roman_numerals(2), "II");
        assert_eq!(roman_numerals(3), "III");
        assert_eq!(roman_numerals(4), "IV");
        assert_eq!(roman_numerals(5), "V");
        assert_eq!(roman_numerals(6), "VI");
        assert_eq!(roman_numerals(7), "VII");
        assert_eq!(roman_numerals(8), "VIII");
        assert_eq!(roman_numerals(9), "IX");
        assert_eq!(roman_numerals(10), "X");
    }

    #[test]
    fn test_11_to_20() {
        assert_eq!(roman_numerals(11), "XI");
        assert_eq!(roman_numerals(12), "XII");
        assert_eq!(roman_numerals(13), "XIII");
        assert_eq!(roman_numerals(14), "XIV");
        assert_eq!(roman_numerals(15), "XV");
        assert_eq!(roman_numerals(16), "XVI");
        assert_eq!(roman_numerals(17), "XVII");
        assert_eq!(roman_numerals(18), "XVIII");
        assert_eq!(roman_numerals(19), "XIX");
        assert_eq!(roman_numerals(20), "XX");
    }
}
