pub fn str_str(haystack: String, needle: String) -> i32 {
    let len_needle = needle.len();
    if len_needle == 0 {
        return 0;
    }

    let len_hay = haystack.len();

    if len_hay == 0 || len_hay < len_needle {
        return -1;
    }

    for i in 0..haystack.len() - (len_needle - 1) {
        let sub = &haystack[i..i + len_needle];
        if sub == needle {
            return i as i32;
        }
    }

    return -1;
}

#[cfg(test)]
mod test {
    use crate::str_str::*;

    #[test]
    fn should_return_0_for_empty_needle() {
        let actual = str_str(String::from("foo"), String::from(""));
        assert_eq!(0, actual);
    }

    #[test]
    fn should_return_minus_one_for_empty_string() {
        let actual = str_str(String::from(""), String::from("foo"));
        assert_eq!(-1, actual);
    }

    #[test]
    fn should_return_0_for_needle_equals_haystack() {
        let actual = str_str(String::from("foo"), String::from("foo"));
        assert_eq!(0, actual);
    }

    #[test]
    fn should_handle_needle_at_end_of_haystack() {
        let actual = str_str(String::from("foobar"), String::from("r"));
        assert_eq!(5, actual);
    }

    #[test]
    fn should_handle_needle_at_foo() {
        let actual = str_str(String::from("abb"), String::from("abaaa"));
        assert_eq!(-1, actual);
    }
}
