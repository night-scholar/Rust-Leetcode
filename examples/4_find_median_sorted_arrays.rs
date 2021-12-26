impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums = nums1;
        let mut nums2 = nums2;
        nums.append(&mut nums2);
        nums.sort_unstable();
        match nums.len()%2{
            //不能用f64除以int类型
            0=>{return (nums[nums.len()/2-1]+nums[nums.len()/2]) as f64/2.},
            1=>{return nums[nums.len()/2] as f64},
            _=>{return 0.}
        }
    }
}