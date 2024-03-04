#[cfg(test)]
mod bmi {
    use crate::bmi_calc;

    #[test]
    fn dummy() {
        let result = 1 + 2;
        assert_eq!(result, 3);
    }

    #[test]
    fn test_calc() {
        let result = bmi_calc(180, 65);
        assert_eq!(result, 20.1);
    }
}

fn bmi_calc<T, U>(height: T, weight: U) -> f64
    where
        T: Into<f64>,
        U: Into<f64>,
{
    let h = height.into() / 100.0;
    let bmi = weight.into() / (h * h);

    (bmi * 10.0).round() / 10.0
}

fn main() {}