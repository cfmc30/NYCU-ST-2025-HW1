pub struct Calc;

impl Calc {
    pub fn add(&self, left: i64, right: i64) -> i64 {
        left + right
    }

    pub fn sub(&self, left: i64, right: i64) -> i64 {
        left - right
    }

    pub fn mul(&self, left: i64, right: i64) -> i64 {
        left * right
    }

    pub fn div(&self, left: i64, right: i64) -> i64 {
        left / right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let calc = Calc;
        let result = calc.add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_sub() {
        let calc = Calc;
        assert_eq!(calc.sub(3, 2), 1);
        assert_eq!(calc.sub(2, 3), -1);
        assert_eq!(calc.sub(2, 2), 0);
    }

    #[test]
    fn test_mul() {
        let calc = Calc;
        assert_eq!(calc.mul(0, 0), 0);
        assert_eq!(calc.mul(1, 0), 0);
        assert_eq!(calc.mul(0, 1), 0);
        assert_eq!(calc.mul(3, 2), 6);
        assert_eq!(calc.mul(3, -2), -6);
    }

    #[test]
    fn test_div() {
        let calc = Calc;
        assert_eq!(calc.div(10, 2), 5);
        assert_eq!(calc.div(3, 2), 1);
        assert_eq!(calc.div(0, 1), 0);
        assert_eq!(calc.div(9, 3), 3);
        assert_eq!(calc.div(10, -2), -5);
    }
    
    #[should_panic]
    #[test]
    fn test_dev_zero() {
        let calc = Calc;
        calc.div(10, 0);
    }
}
