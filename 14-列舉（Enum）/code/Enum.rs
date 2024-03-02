#[allow(dead_code)]
enum CatBreed {
    Persian,             // 波斯貓
    AmericanShorthair,   // 美國短毛貓
    Mix(String, u8),     // 米克斯
}

fn main() {
    let kitty = CatBreed::Mix(String::from("Kitty"), 8);
    let nancy = CatBreed::Persian;

    greeting(&kitty);
    greeting(&nancy);
}

fn greeting(cat: &CatBreed) {
    match cat {
        CatBreed::Mix(name, age) => println!("我是米克斯，我叫 {}，我今年 {} 歲", name, age),
        _ => println!("我是品種貓")
    }
}