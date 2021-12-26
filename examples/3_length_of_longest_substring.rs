// a b c a b c b b -> a -> a b -> a b c -> b c a -> c a b -> a b c -> c b -> b

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        //string类型转数组
        let seq: Vec<char> = s.chars().collect();
        let len = seq.len();
        let (mut start, mut end, mut max) = (0, 0, 0);

        while end < len {
            for idx in start..end {
                if seq[end] == seq[idx] {
                    //删除前面与end位相同的字符
                    start = idx + 1;
                    break;
                }
            }
            let curr = end - start + 1;
            if curr > max {
                max = curr
            }
            end += 1
        }
        max as i32
    }
}