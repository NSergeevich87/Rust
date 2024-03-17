enum tipeOfWine {
    Red,
    White,
    Rose,
}

fn wineTipe(wine: tipeOfWine) {
    match wine {
        tipeOfWine::Red => println!("This is a red wine"),
        _ => println!("This is not a red wine"),
    }
}

fn main() {
    let wine_1 = tipeOfWine::Red;
    wineTipe(wine_1);

    println!("Hello, world!");
}
