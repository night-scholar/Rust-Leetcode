impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut input:i64 = x as i64;
        let mut result:i64=0 ;
        let base :i64 = 2;
        while input != 0{
            result = result * 10 + input % 10;
            input = input / 10;
        }
        if result < -base.pow(31) ||result > base.pow(31) - 1 {
            return 0;
        }
        result as i32
    }
}