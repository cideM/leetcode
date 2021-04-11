// This is such a waste of time. The description of this exercise was so confusing that I thought
// this problem would require dynamic programming, when it ended up being the simplest of all
// loops.
pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() <= 1 {
        return 0;
    }

    let mut max = 0;
    for i in 1..prices.len() {
        if prices[i] > prices[i - 1] {
            max += prices[i] - prices[i - 1];
        }
    }
    max
}

#[cfg(test)]
mod test {
    use crate::max_profit::*;

    #[test]
    fn test1() {
        assert_eq!(0, max_profit(vec![1]));
    }

    #[test]
    fn test2() {
        assert_eq!(0, max_profit(vec![]));
    }

    #[test]
    fn test3() {
        assert_eq!(1, max_profit(vec![0,1]));
    }

    #[test]
    fn test4() {
        assert_eq!(5, max_profit(vec![0,1,5]));
    }

    #[test]
    fn test5() {
        assert_eq!(7, max_profit(vec![7,1,5,3,6,4]));
    }

    #[test]
    fn test6() {
        assert_eq!(0, max_profit(vec![7,6,4,3,1]));
    }

    #[test]
    fn test7() {
        assert_eq!(10, max_profit(vec![5,1,2,1,2,10]));
    }
}
