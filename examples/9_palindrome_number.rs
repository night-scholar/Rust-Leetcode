impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0{
            return false;
        }
        let mut input:i32 = x as i32;
        let mut result:i32=0 ;
        while input != 0{
            result = result * 10 + input % 10;
            input = input / 10;
        }
        if result == x{
            return true;
        }
        false
    }
}