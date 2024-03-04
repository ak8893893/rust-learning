pub fn bmi<T,U>(height: T, weight:U) -> Result<f64,String>
where
    T: Into<f64>,
    U: Into<f64>,
{
    let h = height.into();
    let w = weight.into();

    if h <= 0.0 || w <=0.0{
        return Err("輸入值有誤".to_string());
    }

    let bmi = w /(h/100.0*h/100.0);
    Ok((bmi * 10.0).round() / 10.0)
}