impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();

        for i in (0..nums.len()).step_by(2) {
            if i == nums.len() - 1 || nums[i] != nums[i + 1] {
                return nums[i];
            }
        }

        0
    }
}