use std::io;

pub fn ask_question(question: &str) -> f64{
    println!("{}",question);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("無法讀取輸入");
    input.trim().parse().expect("無法轉換")

}