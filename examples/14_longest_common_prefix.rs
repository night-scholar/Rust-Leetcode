impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut i = 0;
        'over: while i >= 0 {
            let mut b = 0;
            for str in strs.iter() {
                if i >= str.len() {
                    break 'over;
                }
                if b == 0 {
                    b = str.as_bytes()[i].clone();
                }else if b != str.as_bytes()[i] {
                    break 'over;
                }
            }
            i += 1;
        }
        return strs[0][..i].to_string()
    }
}
