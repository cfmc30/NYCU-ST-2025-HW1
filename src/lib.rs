pub struct Calc;

impl Calc {
    pub fn add(&self, left: i64, right: i64) -> i64 {
        left + right
    }

    pub fn sub(&self, left: i64, right: i64) -> i64 {
        left - right
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
}
