pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }

    let mut i = 0;

    for j in 1..nums.len() {
        if nums[j] == nums[i] {
            continue;
        }

        i = i + 1;

        nums.swap(i, j);
    }

    i = i + 1;

    return i as i32;
}

#[cfg(test)]
mod test {
    use crate::remove_duplicates;

    #[test]
    fn should_return_unique_vec() {
        let mut input = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let actual = remove_duplicates::remove_duplicates(&mut input);
        assert_eq!(vec![0, 1, 2, 3, 4], &input[0..actual as usize]);
        assert_eq!(5, actual);
    }

    #[test]
    fn should_handle_empty() {
        let mut input: Vec<i32> = vec![];
        let actual = remove_duplicates::remove_duplicates(&mut input);
        let expected: Vec<i32> = vec![];
        assert_eq!(expected, input);
        assert_eq!(0, actual);
    }

    #[test]
    fn should_handle_vec_of_len_one() {
        let mut input: Vec<i32> = vec![1];
        let actual = remove_duplicates::remove_duplicates(&mut input);
        assert_eq!(vec![1], input);
        assert_eq!(1, actual);
    }
}
