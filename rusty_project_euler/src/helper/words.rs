pub fn number_letter_count(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }

    let mut count = 0;

    let ones = [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let teens = [
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];
    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    if n >= 1000 {
        count += number_letter_count(n / 1000) + 8; // 'thousand' has 8 letters
        count += number_letter_count(n % 1000);
    } else if n >= 100 {
        count += number_letter_count(n / 100) + 7; // 'hundred' has 7 letters
        if !n.is_multiple_of(100) {
            count += 3; // 'and' has 3 letters
            count += number_letter_count(n % 100);
        }
    } else if n >= 20 {
        count += tens[(n / 10) as usize].len() as u32;
        count += number_letter_count(n % 10);
    } else if n >= 10 {
        count += teens[(n - 10) as usize].len() as u32;
    } else {
        count += ones[n as usize].len() as u32;
    }

    count
}
