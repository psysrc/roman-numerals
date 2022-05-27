pub fn roman_numerals(mut num: u32) -> String {
    if num == 0 {
        return String::from("Nulla");
    }

    let mut result = String::from("");

    if num >= 90 {
        if num >= 100 {
            result.push_str("C");
            num -= 100;
        } else {
            result.push_str("XC");
            num -= 90;
        }
    }

    if num >= 40 {
        if num >= 50 {
            result.push_str("L");
            num -= 50;
        } else {
            result.push_str("XL");
            num -= 40;
        }
    }

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

    #[test]
    fn test_40_41_45_49() {
        assert_eq!(roman_numerals(40), "XL");
        assert_eq!(roman_numerals(41), "XLI");
        assert_eq!(roman_numerals(45), "XLV");
        assert_eq!(roman_numerals(49), "XLIX");
    }

    #[test]
    fn test_50_51_54_57_59() {
        assert_eq!(roman_numerals(50), "L");
        assert_eq!(roman_numerals(51), "LI");
        assert_eq!(roman_numerals(54), "LIV");
        assert_eq!(roman_numerals(57), "LVII");
        assert_eq!(roman_numerals(59), "LIX");
    }

    #[test]
    fn test_60_65_70_75_80_85() {
        assert_eq!(roman_numerals(60), "LX");
        assert_eq!(roman_numerals(65), "LXV");
        assert_eq!(roman_numerals(70), "LXX");
        assert_eq!(roman_numerals(75), "LXXV");
        assert_eq!(roman_numerals(80), "LXXX");
        assert_eq!(roman_numerals(85), "LXXXV");
    }

    #[test]
    fn test_90_91_95_98_99() {
        assert_eq!(roman_numerals(90), "XC");
        assert_eq!(roman_numerals(91), "XCI");
        assert_eq!(roman_numerals(95), "XCV");
        assert_eq!(roman_numerals(98), "XCVIII");
        assert_eq!(roman_numerals(99), "XCIX");
    }
}
