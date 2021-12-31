impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len()-1;
        let mut output:i32 = 0;
        while left<right{
            let left_val = height[left];
            let right_val = height[right];
            let area_val = (right-left) as i32 * left_val.min(right_val);
            if area_val > output{
                output = area_val;
            }
            if left_val < right_val{
                left+=1;
            }else{
                right-=1;
            }
        }
        return output;
    }
}
