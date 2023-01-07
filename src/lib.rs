#[derive(PartialEq, Debug)]
pub enum RomanNumeralError {
    ValueTooLarge,
}

pub fn roman_numerals(num: u16) -> Result<String, RomanNumeralError> {
    if num == 0 {
        return Ok(String::from("Nulla"));
    }

    if num > 3999 {
        return Err(RomanNumeralError::ValueTooLarge);
    }

    Ok(convert_to_numerals(num))
}

struct RomanNumeral {
    character: char,
    value: u16,
    can_subtract_from_next: bool,
}

const NUMERALS: [RomanNumeral; 7] = [
    RomanNumeral { character: 'M', value: 1000, can_subtract_from_next: false },
    RomanNumeral { character: 'D', value: 500, can_subtract_from_next: false },
    RomanNumeral { character: 'C', value: 100, can_subtract_from_next: true },
    RomanNumeral { character: 'L', value: 50, can_subtract_from_next: false },
    RomanNumeral { character: 'X', value: 10, can_subtract_from_next: true },
    RomanNumeral { character: 'V', value: 5, can_subtract_from_next: false },
    RomanNumeral { character: 'I', value: 1, can_subtract_from_next: true },
];

fn convert_to_numerals(mut num: u16) -> String {
    let mut result = String::new();

    for numeral_index in 0..(NUMERALS.len() - 1) {
        let mut subtract_index = numeral_index + 1;
        while !(&NUMERALS[subtract_index].can_subtract_from_next) {
            subtract_index += 1;
        }

        result += &process_numeral(&mut num, &NUMERALS[numeral_index], Some(&NUMERALS[subtract_index]));
    }

    result += &process_numeral(&mut num, &NUMERALS[NUMERALS.len() - 1], None);

    result
}

fn process_numeral(num: &mut u16, numeral: &RomanNumeral, prev_numeral: Option<&RomanNumeral>) -> String {
    let mut result = String::new();

    while *num >= numeral.value {
        result.push(numeral.character);
        *num -= numeral.value;
    }

    if let Some(prev_numeral) = prev_numeral {
        let numeral_val_diff = numeral.value - prev_numeral.value;

        if *num >= numeral_val_diff {
            result.push(prev_numeral.character);
            result.push(numeral.character);
            *num -= numeral_val_diff;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(roman_numerals(0), Ok("Nulla".to_string()));
    }

    #[test]
    fn test_1_to_10() {
        assert_eq!(roman_numerals(1), Ok("I".to_string()));
        assert_eq!(roman_numerals(2), Ok("II".to_string()));
        assert_eq!(roman_numerals(3), Ok("III".to_string()));
        assert_eq!(roman_numerals(4), Ok("IV".to_string()));
        assert_eq!(roman_numerals(5), Ok("V".to_string()));
        assert_eq!(roman_numerals(6), Ok("VI".to_string()));
        assert_eq!(roman_numerals(7), Ok("VII".to_string()));
        assert_eq!(roman_numerals(8), Ok("VIII".to_string()));
        assert_eq!(roman_numerals(9), Ok("IX".to_string()));
        assert_eq!(roman_numerals(10), Ok("X".to_string()));
    }

    #[test]
    fn test_11_to_20() {
        assert_eq!(roman_numerals(11), Ok("XI".to_string()));
        assert_eq!(roman_numerals(12), Ok("XII".to_string()));
        assert_eq!(roman_numerals(13), Ok("XIII".to_string()));
        assert_eq!(roman_numerals(14), Ok("XIV".to_string()));
        assert_eq!(roman_numerals(15), Ok("XV".to_string()));
        assert_eq!(roman_numerals(16), Ok("XVI".to_string()));
        assert_eq!(roman_numerals(17), Ok("XVII".to_string()));
        assert_eq!(roman_numerals(18), Ok("XVIII".to_string()));
        assert_eq!(roman_numerals(19), Ok("XIX".to_string()));
        assert_eq!(roman_numerals(20), Ok("XX".to_string()));
    }

    #[test]
    fn test_40s() {
        assert_eq!(roman_numerals(40), Ok("XL".to_string()));
        assert_eq!(roman_numerals(41), Ok("XLI".to_string()));
        assert_eq!(roman_numerals(45), Ok("XLV".to_string()));
        assert_eq!(roman_numerals(49), Ok("XLIX".to_string()));
    }

    #[test]
    fn test_50s() {
        assert_eq!(roman_numerals(50), Ok("L".to_string()));
        assert_eq!(roman_numerals(51), Ok("LI".to_string()));
        assert_eq!(roman_numerals(54), Ok("LIV".to_string()));
        assert_eq!(roman_numerals(57), Ok("LVII".to_string()));
        assert_eq!(roman_numerals(59), Ok("LIX".to_string()));
    }

    #[test]
    fn test_60s_70s_80s() {
        assert_eq!(roman_numerals(60), Ok("LX".to_string()));
        assert_eq!(roman_numerals(65), Ok("LXV".to_string()));
        assert_eq!(roman_numerals(70), Ok("LXX".to_string()));
        assert_eq!(roman_numerals(75), Ok("LXXV".to_string()));
        assert_eq!(roman_numerals(80), Ok("LXXX".to_string()));
        assert_eq!(roman_numerals(85), Ok("LXXXV".to_string()));
    }

    #[test]
    fn test_90s() {
        assert_eq!(roman_numerals(90), Ok("XC".to_string()));
        assert_eq!(roman_numerals(91), Ok("XCI".to_string()));
        assert_eq!(roman_numerals(95), Ok("XCV".to_string()));
        assert_eq!(roman_numerals(98), Ok("XCVIII".to_string()));
        assert_eq!(roman_numerals(99), Ok("XCIX".to_string()));
    }

    #[test]
    fn test_100s() {
        assert_eq!(roman_numerals(100), Ok("C".to_string()));
        assert_eq!(roman_numerals(104), Ok("CIV".to_string()));
        assert_eq!(roman_numerals(110), Ok("CX".to_string()));
        assert_eq!(roman_numerals(120), Ok("CXX".to_string()));
        assert_eq!(roman_numerals(130), Ok("CXXX".to_string()));
        assert_eq!(roman_numerals(140), Ok("CXL".to_string()));
        assert_eq!(roman_numerals(150), Ok("CL".to_string()));
        assert_eq!(roman_numerals(170), Ok("CLXX".to_string()));
        assert_eq!(roman_numerals(190), Ok("CXC".to_string()));
        assert_eq!(roman_numerals(199), Ok("CXCIX".to_string()));
    }

    #[test]
    fn test_200s() {
        assert_eq!(roman_numerals(200), Ok("CC".to_string()));
        assert_eq!(roman_numerals(210), Ok("CCX".to_string()));
        assert_eq!(roman_numerals(249), Ok("CCXLIX".to_string()));
        assert_eq!(roman_numerals(250), Ok("CCL".to_string()));
        assert_eq!(roman_numerals(270), Ok("CCLXX".to_string()));
        assert_eq!(roman_numerals(290), Ok("CCXC".to_string()));
        assert_eq!(roman_numerals(299), Ok("CCXCIX".to_string()));
    }

    #[test]
    fn test_300s() {
        assert_eq!(roman_numerals(300), Ok("CCC".to_string()));
        assert_eq!(roman_numerals(323), Ok("CCCXXIII".to_string()));
        assert_eq!(roman_numerals(350), Ok("CCCL".to_string()));
        assert_eq!(roman_numerals(370), Ok("CCCLXX".to_string()));
        assert_eq!(roman_numerals(399), Ok("CCCXCIX".to_string()));
    }

    #[test]
    fn test_400s() {
        assert_eq!(roman_numerals(400), Ok("CD".to_string()));
        assert_eq!(roman_numerals(491), Ok("CDXCI".to_string()));
        assert_eq!(roman_numerals(450), Ok("CDL".to_string()));
        assert_eq!(roman_numerals(470), Ok("CDLXX".to_string()));
        assert_eq!(roman_numerals(499), Ok("CDXCIX".to_string()));
    }

    #[test]
    fn test_500s() {
        assert_eq!(roman_numerals(500), Ok("D".to_string()));
        assert_eq!(roman_numerals(569), Ok("DLXIX".to_string()));
        assert_eq!(roman_numerals(587), Ok("DLXXXVII".to_string()));
        assert_eq!(roman_numerals(565), Ok("DLXV".to_string()));
        assert_eq!(roman_numerals(520), Ok("DXX".to_string()));
    }

    #[test]
    fn test_600s_700s_800s_900s() {
        assert_eq!(roman_numerals(600), Ok("DC".to_string()));
        assert_eq!(roman_numerals(651), Ok("DCLI".to_string()));
        assert_eq!(roman_numerals(824), Ok("DCCCXXIV".to_string()));
        assert_eq!(roman_numerals(958), Ok("CMLVIII".to_string()));
        assert_eq!(roman_numerals(737), Ok("DCCXXXVII".to_string()));
        assert_eq!(roman_numerals(937), Ok("CMXXXVII".to_string()));
        assert_eq!(roman_numerals(835), Ok("DCCCXXXV".to_string()));
        assert_eq!(roman_numerals(731), Ok("DCCXXXI".to_string()));
        assert_eq!(roman_numerals(999), Ok("CMXCIX".to_string()));
    }

    #[test]
    fn test_1000_to_3999() {
        assert_eq!(roman_numerals(1000), Ok("M".to_string()));
        assert_eq!(roman_numerals(1557), Ok("MDLVII".to_string()));
        assert_eq!(roman_numerals(1177), Ok("MCLXXVII".to_string()));
        assert_eq!(roman_numerals(2249), Ok("MMCCXLIX".to_string()));
        assert_eq!(roman_numerals(2664), Ok("MMDCLXIV".to_string()));
        assert_eq!(roman_numerals(3377), Ok("MMMCCCLXXVII".to_string()));
        assert_eq!(roman_numerals(3999), Ok("MMMCMXCIX".to_string()));
    }

    #[test]
    fn test_more_than_3999() {
        assert_eq!(roman_numerals(4000), Err(RomanNumeralError::ValueTooLarge));
        assert_eq!(roman_numerals(65535), Err(RomanNumeralError::ValueTooLarge));
    }
}
