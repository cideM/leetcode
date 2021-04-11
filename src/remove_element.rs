pub fn remove_element(nums: &mut Vec<i32>, value: i32) -> i32 {
    let mut i = 0;
    for j in 0..nums.len() {
        if nums[j] != value {
            nums[i] = nums[j];
            i = i + 1;
        }
    }

    return i as i32
}

#[cfg(test)]
mod test {
    use crate::remove_element;

    #[test]
    fn should_remove_consecutive_elements() {
        let mut input = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let actual = remove_element::remove_element(&mut input, 1);
        assert_eq!(vec![0, 0, 2, 2, 3, 3, 4], &input[0..actual as usize]);
        assert_eq!(7, actual);
    }

    #[test]
    fn should_remove_elements_at_start() {
        let mut input = vec![1, 2];
        let actual = remove_element::remove_element(&mut input, 1);
        assert_eq!(vec![2], &input[0..actual as usize]);
        assert_eq!(1, actual);
    }

    #[test]
    fn should_handle_empty() {
        let mut input: Vec<i32> = vec![];
        let actual = remove_element::remove_element(&mut input, 1);
        let expected: Vec<i32> = vec![];
        assert_eq!(expected, input);
        assert_eq!(0, actual);
    }

    #[test]
    fn should_handle_vec_of_len_one() {
        let mut input: Vec<i32> = vec![1];
        let actual = remove_element::remove_element(&mut input, 2);
        assert_eq!(vec![1], input);
        assert_eq!(1, actual);
    }

    #[test]
    fn should_remove_only_element() {
        let mut input: Vec<i32> = vec![1];
        let actual = remove_element::remove_element(&mut input, 1);
        let expected: Vec<i32> = vec![];
        assert_eq!(expected, &input[0..actual as usize]);
        assert_eq!(0, actual);
    }
}
