struct Skill {
    action: String
}

enum CatBreed {
    Persian,             // 波斯貓
    AmericanShorthair,   // 美國短毛貓
    Mix(String, u8),     // 米克斯
    Other(Skill),        // 其它
    Alien{power: u32}    // 外星貓
}

fn main() {
    let goku_cat = CatBreed::Other(Skill{action: "龜派氣功".to_string()});
    let frieza_cat = CatBreed::Alien { power: 530000 }; // 戰鬥力 53 萬

    greeting(&goku_cat);
    greeting(&frieza_cat);
}

fn greeting(cat: &CatBreed) {
    match cat {
        CatBreed::Mix(name, age) => println!("我是米克斯，我叫 {}，我今年 {} 歲", name, age),
        CatBreed::Other(skill) => println!("使出絕招{}！", skill.action),
        CatBreed::Alien { power } => println!("我的戰鬥力是 {}", power),
        _ => println!("我是品種貓")
    }
}