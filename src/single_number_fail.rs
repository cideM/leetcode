pub fn single_number_fail(why: Vec<i32>) -> i32 {
    let mut nums = why.clone();
    let len = nums.len();

    if len == 1 {
        return nums[0];
    }

    let mut i = 0;
    let mut j = 1;

    while j < len {
        if nums[j] != nums[i] {
            j = j + 1;
            continue;
        }

        if nums[j] == nums[i] && j - i == 1 {
            if j == len - 2 {
                i = i + 2;
                break;
            }
            i = i + 2;
            j = j + 2;
            continue;
        }

        nums.swap(i + 1, j);
        i = i + 2;
        j = i + 1;
    }

    return nums[i];
}

#[cfg(test)]
mod test {
    use crate::single_number_fail::*;

    #[test]
    fn whyyyyyyyyy() {
        let input = vec![4, 1, 2, 1, 2];
        assert_eq!(4, single_number_fail(input));
    }

    #[test]
    fn whyyyy() {
        let input = vec![2,2,1];
        assert_eq!(1, single_number_fail(input));
    }

    #[test]
    fn whyyy() {
        let input = vec![4,4,2,1,1,2,5];
        assert_eq!(5, single_number_fail(input));
    }
}
