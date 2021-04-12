pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    if len < 2 {
        return;
    };
    let k = k % len as i32;
    if k == 0 {
        return;
    };
    reverse_vec(nums, 0, len - 1);
    reverse_vec(nums, 0, (k as usize) - 1);
    reverse_vec(nums, k as usize, len - 1);
}

fn reverse_vec<T>(v: &mut Vec<T>, start: usize, end: usize) {
    if start < end {
        let mut other = end;
        for i in start..start + (end - start) / 2 + 1 {
            v.swap(i, other);
            other -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::rotate::*;

    #[test]
    fn reverse() {
        let mut input = vec![1, 2];
        let expected = vec![2, 1];
        reverse_vec(&mut input, 0, 1);
        assert_eq!(expected, input);
    }

    #[test]
    fn reverse4() {
        let mut input = vec![1, 2];
        let expected = vec![1, 2];
        reverse_vec(&mut input, 1, 1);
        assert_eq!(expected, input);
    }

    #[test]
    fn reverse2() {
        let mut input = vec![1, 2, 3, 4];
        let expected = vec![1, 2, 4, 3];
        reverse_vec(&mut input, 2, 3);
        assert_eq!(expected, input);
    }

    #[test]
    fn reverse3() {
        let mut input = vec![1, 2, 3, 4];
        let expected = vec![4, 3, 2, 1];
        reverse_vec(&mut input, 0, 3);
        assert_eq!(expected, input);
    }

    #[test]
    fn test1() {
        let mut input = vec![1, 2, 3, 4, 5, 6, 7];
        let expected = vec![5, 6, 7, 1, 2, 3, 4];
        rotate(&mut input, 3);
        assert_eq!(expected, input);
    }

    #[test]
    fn test2() {
        let mut input = vec![1, 2];
        let expected = vec![1, 2];
        rotate(&mut input, 2);
        assert_eq!(expected, input);
    }

    #[test]
    fn test3() {
        let mut input = vec![1, 2];
        let expected = vec![2, 1];
        rotate(&mut input, 1);
        assert_eq!(expected, input);
    }

    #[test]
    fn test5() {
        let mut input = vec![1, 2];
        let expected = vec![2, 1];
        rotate(&mut input, 3);
        assert_eq!(expected, input);
    }

    #[test]
    fn test4() {
        let mut input: Vec<i32> = vec![];
        let expected: Vec<i32> = vec![];
        rotate(&mut input, 100);
        assert_eq!(expected, input);
    }
}
