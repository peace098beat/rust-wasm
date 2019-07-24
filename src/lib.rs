#[no_mangle]
pub fn sum_from1_to10() -> i32 {
    let mut sum = 0;
    for i in 1..=10 {
        sum += i;
    }
    sum
}

#[no_mangle]
pub fn twice(value: f64) -> f64 {
    value * 2.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_from1_to10() {
        let sum = sum_from1_to10();
        assert_eq!(sum, 55);
    }

    
    #[test]
    fn test_twice() {
        let ans = twice(5.0);
        assert_eq!(ans, 10.0);
    }

}
