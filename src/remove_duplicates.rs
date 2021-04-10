pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    nums.len() as i32
}

#[cfg(test)]
mod test {
    use crate::remove_duplicates;

    #[test]
    fn should_return_2() {
        let mut input = vec![1,1,2];
        let actual = remove_duplicates::remove_duplicates(&mut input);
        assert_eq!(2, actual);
        assert_eq!(input, vec![1,2]);
    }
}
