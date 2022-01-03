impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let table: Vec<(i32, &'static str)> = vec![
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];
        let mut sum = 0;
        let mut idx = 0;
        for p in table.iter() {
            while idx + p.1.len() <= s.len() && p.1 == &s[idx..idx + p.1.len()] {
                idx += p.1.len();
                sum += p.0;
                if idx >= s.len() {
                    return sum;
                }
            }
        }
        sum
    }
}