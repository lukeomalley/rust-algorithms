use std::collections::HashMap;

// Given an array of integers nums and an integer target, return indices of
// the two numbers such that they add up to target.

// You may assume that each input would have exactly one solution, and you may
// not use the same element twice.

// You can return the answer in any order.

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
	let mut targets = HashMap::new();

	for (i, num) in nums.iter().enumerate() {
		let found = targets.contains_key(num);

		if found {
			let found_num = targets.get(num).unwrap();
			return vec![*found_num, i as i32];
		}

		targets.insert(target - num, i as i32);
	}

	return vec![];
}

#[test]
fn works() {
	let nums: Vec<i32> = vec![1, 7, 11, 15];
	let target = 9;
	let expected: Vec<i32> = vec![2, 7];

	assert_eq!(two_sum(nums, target), expected)
}
