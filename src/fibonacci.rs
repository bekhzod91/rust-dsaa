pub fn fiboncci() -> Box<dyn FnMut() -> i32> {
    let (mut a, mut b) = (0, 1);
    Box::new(move || {
        let tmp = a;
        (a, b) = (b, a + b);   
        return tmp;

    }) 
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonnaci_number() {
        let mut fib = fiboncci();

        assert_eq!(0, fib());
        assert_eq!(1, fib());
        assert_eq!(1, fib());
        assert_eq!(2, fib());
        assert_eq!(3, fib());
        assert_eq!(5, fib());
        assert_eq!(8, fib());
    }
}