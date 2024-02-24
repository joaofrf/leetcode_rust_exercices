/*

1. Two Sum
Solved
Easy
Topics
Companies
Hint
Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.


*/

fn main() {

    let nums = vec![2,7,11,15];
    let target = 9;

    println!("{:?}", Solution::two_sum(nums, target));
}

struct Solution{

}


impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        for (i_index, i) in nums.iter().enumerate(){
            for (x_index, x) in nums.iter().enumerate(){
                if i_index == x_index{
                    continue;
                }

                if i+x==target {
                    return vec![i_index.try_into().unwrap(), x_index.try_into().unwrap()]
                }

            }
        }
        vec![]
    }
}