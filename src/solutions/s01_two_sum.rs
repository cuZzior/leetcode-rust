use std::collections::HashMap;

pub fn brute_force(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for f in 0..nums.len() {
        for g in (f + 1)..nums.len() {
            if (nums[f] + nums[g]) == target {
                return vec![f as i32, g as i32];
            }
        }
    }
    vec![]
}

pub fn legit(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen = HashMap::<i32, usize>::new();
    for (idx, element) in nums.iter().enumerate() {
        let needed = target - element;
        if let Some(res) = seen.get(&needed) {
            return vec![*res as i32, idx as i32];
        }
        seen.insert(*element, idx);
    }
    vec![]
}

pub fn idiomatic(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut seen = HashMap::with_capacity(nums.len());
    for (idx, &val) in nums.iter().enumerate() {
        if let Some(&prev) = seen.get(&(target - val)) {
            return Some((prev, idx));
        }
        seen.insert(val, idx);
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_two_sum() {
        let nums: Vec<i32> = vec![2, 7, 11, 15, 16, 17, 19, 50, 51, 55, 70];
        let target: i32 = 36;
        let exp: Vec<i32> = vec![5, 6];

        assert_eq!(brute_force(nums.clone(), target), exp);
        assert_eq!(legit(nums, target), exp);
    }

    #[test]
    fn test_two_sum_idiomatic() {
        let nums = [2, 7, 11, 15, 16, 17, 19, 50, 51, 55, 70];
        assert_eq!(idiomatic(&nums, 36), Some((5, 6)));
        assert_eq!(idiomatic(&nums, 9), Some((0, 1)));
        assert_eq!(idiomatic(&nums, 1), None);
        assert_eq!(idiomatic(&[], 5), None);
    }
}
