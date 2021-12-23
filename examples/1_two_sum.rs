use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        //创建一个可变的hashmap
        let mut map = HashMap::new();
        //迭代nums数组，i为key，item为value，要获取其引用,enumerate相当于将迭代的结果返回
        for (index, num) in nums.iter().enumerate() {
            //取key为target - num的值
            match map.get(&(target - num)) {
                //不存在
                None => {
                    //添加到hashmap
                    map.insert(num, index);
                }
                //存在返回下标
                Some(sub_index) => return vec![*sub_index as i32, index as i32],
            }
        }
        //不存在返回空
        vec![]
    }
}

fn main(){
    assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
}