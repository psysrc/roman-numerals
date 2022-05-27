pub fn roman_numerals(num: u32) -> &'static str {
    match num {
        0 => "Nulla",
        1 => "I",
        2 => "II",
        3 => "III",
        4 => "IV",
        5 => "V",
        6 => "VI",
        7 => "VII",
        8 => "VIII",
        9 => "IX",
        10 => "X",
        11 => "XI",
        12 => "XII",
        13 => "XIII",
        14 => "XIV",
        15 => "XV",
        16 => "XVI",
        17 => "XVII",
        18 => "XVIII",
        19 => "XIX",
        20 => "XX",
        _ => panic!("Cannot convert number to roman numerals"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_unknown() {
        roman_numerals(271041);
    }

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
