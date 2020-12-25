use std::ops::Rem;

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.trim().len() < 2 || !code.chars().all(|c| c.is_numeric() || c == ' '){
        return false;
    }

    fn cal_even_digits(n: u32) -> u32{
        let n = n * 2;
        if n > 9 {
            n - 9
        }else{
            n
        }
    }

    let n: u32 = code.chars()
        .filter(|c| c.is_numeric())
        .rev()
        .enumerate()
        .map(|(idx, c)|{
            let n = c.to_digit(10).unwrap();
            if idx & 1 == 0{
                n
            }else{
                cal_even_digits(n)
            }
        })
        .sum();
    n.rem(10) == 0
}
