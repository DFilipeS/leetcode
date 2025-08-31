use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut search_map = HashMap::new();

    for (num, index) in nums.iter().zip(0..) {
        let complement = target - num;
        if let Some(&other_index) = search_map.get(&complement) {
            return vec![other_index, index];
        }
        search_map.insert(num, index);
    }

    panic!("Valid answer not found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let cases = vec![
            (vec![2, 7, 11, 15], 9, [0, 1]),
            (vec![3, 2, 4], 6, [1, 2]),
            (vec![3, 3], 6, [0, 1]),
        ];

        for (nums, target, expected) in cases {
            let result = two_sum(nums.clone(), target);
            assert_eq!(result, expected, "Test case {:?}", nums);
        }
    }
}
