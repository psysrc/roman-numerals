fn process_numeral(num: &mut u32, numeral: (char, u32), prev_numeral: (char, u32)) -> String {
    let mut result = String::new();

    while *num >= numeral.1 {
        result.push(numeral.0);
        *num -= numeral.1;
    }

    let numeral_val_diff = numeral.1 - prev_numeral.1;

    if *num >= numeral_val_diff {
        result.push(prev_numeral.0);
        result.push(numeral.0);
        *num -= numeral_val_diff;
    }

    result
}

pub fn roman_numerals(mut num: u32) -> String {
    if num == 0 {
        return String::from("Nulla");
    }

    let mut result = String::new();

    result += &process_numeral(&mut num, ('M', 1000), ('C', 100));
    result += &process_numeral(&mut num, ('D', 500), ('C', 100));
    result += &process_numeral(&mut num, ('C', 100), ('X', 10));
    result += &process_numeral(&mut num, ('L', 50), ('X', 10));
    result += &process_numeral(&mut num, ('X', 10), ('I', 1));
    result += &process_numeral(&mut num, ('V', 5), ('I', 1));
    result += &process_numeral(&mut num, ('I', 1), ('Z', 0));

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
    fn test_40s() {
        assert_eq!(roman_numerals(40), "XL");
        assert_eq!(roman_numerals(41), "XLI");
        assert_eq!(roman_numerals(45), "XLV");
        assert_eq!(roman_numerals(49), "XLIX");
    }

    #[test]
    fn test_50s() {
        assert_eq!(roman_numerals(50), "L");
        assert_eq!(roman_numerals(51), "LI");
        assert_eq!(roman_numerals(54), "LIV");
        assert_eq!(roman_numerals(57), "LVII");
        assert_eq!(roman_numerals(59), "LIX");
    }

    #[test]
    fn test_60s_70s_80s() {
        assert_eq!(roman_numerals(60), "LX");
        assert_eq!(roman_numerals(65), "LXV");
        assert_eq!(roman_numerals(70), "LXX");
        assert_eq!(roman_numerals(75), "LXXV");
        assert_eq!(roman_numerals(80), "LXXX");
        assert_eq!(roman_numerals(85), "LXXXV");
    }

    #[test]
    fn test_90s() {
        assert_eq!(roman_numerals(90), "XC");
        assert_eq!(roman_numerals(91), "XCI");
        assert_eq!(roman_numerals(95), "XCV");
        assert_eq!(roman_numerals(98), "XCVIII");
        assert_eq!(roman_numerals(99), "XCIX");
    }

    #[test]
    fn test_100s() {
        assert_eq!(roman_numerals(100), "C");
        assert_eq!(roman_numerals(104), "CIV");
        assert_eq!(roman_numerals(110), "CX");
        assert_eq!(roman_numerals(120), "CXX");
        assert_eq!(roman_numerals(130), "CXXX");
        assert_eq!(roman_numerals(140), "CXL");
        assert_eq!(roman_numerals(150), "CL");
        assert_eq!(roman_numerals(170), "CLXX");
        assert_eq!(roman_numerals(190), "CXC");
        assert_eq!(roman_numerals(199), "CXCIX");
    }

    #[test]
    fn test_200s() {
        assert_eq!(roman_numerals(200), "CC");
        assert_eq!(roman_numerals(210), "CCX");
        assert_eq!(roman_numerals(249), "CCXLIX");
        assert_eq!(roman_numerals(250), "CCL");
        assert_eq!(roman_numerals(270), "CCLXX");
        assert_eq!(roman_numerals(290), "CCXC");
        assert_eq!(roman_numerals(299), "CCXCIX");
    }

    #[test]
    fn test_300s() {
        assert_eq!(roman_numerals(300), "CCC");
        assert_eq!(roman_numerals(323), "CCCXXIII");
        assert_eq!(roman_numerals(350), "CCCL");
        assert_eq!(roman_numerals(370), "CCCLXX");
        assert_eq!(roman_numerals(399), "CCCXCIX");
    }

    #[test]
    fn test_400s() {
        assert_eq!(roman_numerals(400), "CD");
        assert_eq!(roman_numerals(491), "CDXCI");
        assert_eq!(roman_numerals(450), "CDL");
        assert_eq!(roman_numerals(470), "CDLXX");
        assert_eq!(roman_numerals(499), "CDXCIX");
    }

    #[test]
    fn test_500s() {
        assert_eq!(roman_numerals(500), "D");
        assert_eq!(roman_numerals(569), "DLXIX");
        assert_eq!(roman_numerals(587), "DLXXXVII");
        assert_eq!(roman_numerals(565), "DLXV");
        assert_eq!(roman_numerals(520), "DXX");
    }

    #[test]
    fn test_600s_700s_800s_900s() {
        assert_eq!(roman_numerals(600), "DC");
        assert_eq!(roman_numerals(651), "DCLI");
        assert_eq!(roman_numerals(824), "DCCCXXIV");
        assert_eq!(roman_numerals(958), "CMLVIII");
        assert_eq!(roman_numerals(737), "DCCXXXVII");
        assert_eq!(roman_numerals(937), "CMXXXVII");
        assert_eq!(roman_numerals(835), "DCCCXXXV");
        assert_eq!(roman_numerals(731), "DCCXXXI");
        assert_eq!(roman_numerals(999), "CMXCIX");
    }

    #[test]
    fn test_1000_to_3999() {
        assert_eq!(roman_numerals(1000), "M");
        assert_eq!(roman_numerals(1557), "MDLVII");
        assert_eq!(roman_numerals(1177), "MCLXXVII");
        assert_eq!(roman_numerals(2249), "MMCCXLIX");
        assert_eq!(roman_numerals(2664), "MMDCLXIV");
        assert_eq!(roman_numerals(3377), "MMMCCCLXXVII");
        assert_eq!(roman_numerals(3999), "MMMCMXCIX");
    }
}
