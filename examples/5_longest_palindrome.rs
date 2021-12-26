// 1.先向后找到相同的字符串，标记首尾
// 2.首尾前后移动，判断字符串是否相同，得出长度和开始位置，与最大长度比较
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let seq:Vec<char> = s.chars().collect();
        let len = seq.len();
        let (mut maxlen,mut begin,mut index) = (0,0,0);
        while index < len{
            let (mut start,mut end) = (index,index);
            while end < len-1&&seq[end] == seq[end+1]{
                end += 1;
            }
            while end < len-1 && start > 0 && seq[end+1] == seq[start-1]{
                end += 1;
                start -= 1;
            }
            if end-start+1 > maxlen{
                maxlen = end-start+1;
                begin = start;
            }
            index += 1;
        }
        s[begin..begin+maxlen].to_owned()
    }
}