const EXAMPLE_DATA: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

pub fn day2(data: &str) {
    //let data = EXAMPLE_DATA;

    let sum1 = data
        .split(',')
        .map(|v| {
            let (start, end) = v.trim().split_once('-').unwrap();
            let start = start.parse::<i64>().unwrap();
            let end   = end.parse::<i64>().unwrap();

            (start..=end)
                .map(|i| {
                    let digit_count = f64::floor(1.0 + f64::log10(i as f64)) as u32;
                    let filter = 10_i64.pow(digit_count / 2);
                    if i % filter == i / filter { i } else { 0 }
                })
                .sum::<i64>()
        })
        .sum::<i64>();

    let sum2 = data
        .split(',')
        .map(|v| {
            let (start, end) = v.trim().split_once('-').unwrap();
            let start = start.parse::<i64>().unwrap();
            let end   = end.parse::<i64>().unwrap();

            (start..=end)
                .map(|i| {
                    let string = format!("{}", i);
                    let digit_count = string.len();
                    let invalid = (1..=digit_count / 2).any(|digits| {
                        let pattern = &string.as_bytes()[..digits];
                        string.as_bytes().chunks(digits).all(|part| part == pattern)
                    });
                    if invalid { i } else { 0 }
                })
                .sum::<i64>()
        })
        .sum::<i64>();

    println!("Part 1: {}", sum1);
    println!("Part 2: {}", sum2);
}
