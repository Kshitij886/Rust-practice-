// A subarray is called balanced if the number of distinct even numbers in the subarray is equal to the number of distinct odd numbers.
//Return the length of the longest balanced subarray.

/*
    Input: nums = [3,2,2,5,4]
    Output: 5
    Explanation:
    The longest balanced subarray is [3, 2, 2, 5, 4].
    It has 2 distinct even numbers [2, 4] and 2 distinct odd numbers [3, 5]. Thus, the answer is 5.   [1,2,3,2]
*/
#![allow(unused)]

use core::num;

fn main() {
    println!("hello world");
    let sol = Solution::longest_balanced(vec![3, 2, 2, 5, 4]);
    println!("sol :{}", sol);
}

struct Solution;

impl Solution {
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        let mut even_no: i32 = 0;
        let mut odd_no: i32 = 0;
        let mut longest: i32 = 0;
        (even_no, odd_no)= count(&nums);
        println!("{}, {}", even_no, odd_no);
        if even_no == odd_no  {
            longest = nums.len() as i32;
            println!("{}", longest);
            return longest;
        };
        let nums_iter = nums.iter();
        for n in 0..nums.len() {
            let mut even = 0;
            let mut odd = 0;
            for i in n..nums.len() {
                if nums[i] % 2 == 0 {
                    even += 1;
                } else {
                    odd += 1;
                }
                if even == odd && nums.len() as i32 - i as i32 > longest{
                    longest = nums.len() as i32 - i as i32;
                }
            }
        }
        return longest;
        fn count(vec: &Vec<i32>) -> (i32, i32) {
            let mut even = 0;
            let mut odd = 0;
            for i in vec {
                if i % 2 == 0 {
                    even = even + 1;
                } else {
                    odd = odd + 1;
                }
            }
            return (even, odd);
        }
    }
}
