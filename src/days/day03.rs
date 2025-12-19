pub static INPUT: &str = include_str!("../input/3.txt");
pub static TEST_INPUT: &str = include_str!("../input/3_test.txt");

pub fn a(input: &str) -> i32 {
    let mut max_joltage = 0;

    for line in input.lines() {
        let bank = line.as_bytes();

        let mut large_index = 0;
        let mut large_digit = 0;

        for digit in (b'0'..=b'9').rev() {
            if let Some(index) = bank[..(bank.len() - 1)].iter().position(|d| *d == digit) {
                large_index = index;
                large_digit = digit;
                break;
            }
        }

        let mut second_digit = 0;

        for digit in (b'0'..=b'9').rev() {
            if bank[(large_index + 1)..].contains(&digit) {
                second_digit = digit;
                break;
            }
        }

        max_joltage += (large_digit - b'0') as i32 * 10 + (second_digit - b'0') as i32;
    }

    max_joltage
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 357);
    assert_eq!(a(INPUT), 17142);
}

pub fn b(input: &str) -> i32 {
    0
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 0);
    assert_eq!(b(INPUT), 0);
}
