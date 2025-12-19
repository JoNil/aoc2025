pub static INPUT: &str = include_str!("../input/2.txt");
pub static TEST_INPUT: &str = include_str!("../input/2_test.txt");

pub fn a(input: &str) -> i64 {
    let mut sum = 0;

    for range in input.trim().split(',') {
        let (a, b) = range.split_once('-').unwrap();
        let a = a.parse::<i64>().unwrap();
        let b = b.parse::<i64>().unwrap();

        for num in a..=b {
            let nums = format!("{num}");
            let num_len = nums.len();
            let (a, b) = nums.split_at(num_len / 2);
            if a == b {
                sum += num
            }
        }
    }

    sum
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 1227775554);
    assert_eq!(a(INPUT), 15873079081);
}

pub fn b(input: &str) -> i64 {
    let mut sum = 0;

    for range in input.trim().split(',') {
        let (a, b) = range.split_once('-').unwrap();
        let a = a.parse::<i64>().unwrap();
        let b = b.parse::<i64>().unwrap();

        for num in a..=b {
            let nums = format!("{num}");
            let num_len = nums.len();

            for divisor in 2..=num_len {
                if num_len % divisor == 0 {
                    let mut found = true;
                    let sublen = num_len / divisor;

                    let (first_part, mut rest) = nums.split_at(sublen);

                    for _ in 0..(divisor - 1) {
                        let (part, new_rest) = rest.split_at(sublen);
                        rest = new_rest;

                        if first_part != part {
                            found = false;
                            break;
                        }
                    }

                    if found {
                        sum += num;
                        break;
                    }
                }
            }
        }
    }

    sum
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 4174379265);
    assert_eq!(b(INPUT), 22617871034);
}
