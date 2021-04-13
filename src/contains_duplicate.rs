// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/578/

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    if nums.len() <= 1 {
        return false;
    }

    let mut copy = nums.to_vec();

    merge_sort(&mut copy);

    for i in 0..copy.len() - 1 {
        if copy[i] == copy[i + 1] {
            return true;
        }
    }

    false
}

fn merge_sort(nums: &mut Vec<i32>) {
    let mut array_b = nums.to_vec();
    top_down_split_merge(&mut array_b, 0, nums.len() as i32, nums);
}

fn top_down_split_merge(b: &mut Vec<i32>, start: i32, end: i32, a: &mut Vec<i32>) {
    println!("top_down_split_merge b {:?} start {} end {} a {:?}", b, start, end, a);
    if end - start <= 1 {
        return;
    }

    let middle = (start + end) / 2;

    top_down_split_merge(a, start, middle, b); top_down_split_merge(a, middle, end, b);
    top_down_merge(a, start as usize, middle as usize, end as usize, b);
}

fn top_down_merge(a: &mut Vec<i32>, start: usize, middle: usize, end: usize, b: &mut Vec<i32>) {
    // What's confusing about merge sort is that it's always explained as splitting the original
    // array into smaller arrays. But the algorithm on Wikipedia, which is used here, doesn't
    // actually split anything. The "splitting" is achieved by modifying the start, middle and end
    // indeces. That's why the comparison as to whether the left or the right value is smaller is
    // done on the same array here, but using different indeces (b[i] <= b[j])
    let mut i = start; // i points at the left sublist
    let mut j = middle; // j points at the right sublist

    for k in start..end {
        if i < middle && (j >= end || b[i] <= b[j]) {
            a[k] = b[i];
            i += 1;
        } else {
            a[k] = b[j];
            j += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::contains_duplicate::*;

    #[test]
    fn should_sort_vec() {
        let mut input = vec![3, 2, 1];
        merge_sort(&mut input);
        assert_eq!(vec![1, 2, 3], input);
    }

    // #[test]
    // fn should_return_false_for_empty_vec() {
    //     assert_eq!(false, contains_duplicate(vec![]));
    // }

    // #[test]
    // fn should_return_false_for_single_element_vec() {
    //     assert_eq!(false, contains_duplicate(vec![1]));
    // }

    // #[test]
    // fn should_return_false_if_no_duplicates() {
    //     assert_eq!(false, contains_duplicate(vec![1, 2]));
    // }

    // #[test]
    // fn should_return_true_if_duplicates() {
    //     assert_eq!(true, contains_duplicate(vec![1,2,3,1]));
    // }

    // #[test]
    // fn should_return_true_if_consecutive_duplicates() {
    //     assert_eq!(true, contains_duplicate(vec![1, 2, 1]));
    // }
}
