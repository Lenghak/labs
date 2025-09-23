pub fn is_armstrong_number(num: u32) -> bool {
    let mut result: u32 = 0;
    let length = (num as f32).log10().floor() as u32 + 1;
    let mut c_num = num;

    while c_num > 0 {
        let digit = c_num % 10;
        c_num /= 10;
        result += digit.pow(length)
    }

    result == num
}
