enum DiskType {
    SSD,
    HDD,
}
#[derive(Debug)]
enum DiskSize {
    GB(u32),
    TB(u32),
}

fn main() {
    let disk_1 = DiskType::SSD;
    // let disk_2 = DiskType::HDD;
    match disk_1 {
        DiskType::HDD => println!("Disk 1 is HDD"),
        DiskType::SSD => println!("Disk 1 is SSD"),
    }

    let disk_size = DiskSize::GB(512);
    println!("{:?}", disk_size);

    let wine_1 = Wine {
        name: String::from("Chardonnay"),
        region: RegionsOfWine::France,
        year: 2019,
    };

    let wine_2 = Wine {
        name: String::from("Chlenistoe"),
        region: RegionsOfWine::Chile,
        year: 2018,
    };

    println!("Wine 1: {:?}", wine_1);
    println!("Wine 2: {:?}", wine_2);

    permition_region(wine_1.region);
    permition_region(wine_2.region);

    popularity_region(RegionsOfWine::SouthAfrica);
}

#[derive(Debug)]
enum RegionsOfWine {
    France,
    Italy,
    Spain,
    Chile,
    Argentina,
    USA,
    Australia,
    SouthAfrica,
}

#[derive(Debug)]
struct Wine {
    name: String,
    region: RegionsOfWine,
    year: u32,
}

fn permition_region(region: RegionsOfWine) {
    match region {
        RegionsOfWine::France => println!("You can produce wine in France"),
        RegionsOfWine::Italy => println!("You can produce wine in Italy"),
        RegionsOfWine::Spain => println!("You can produce wine in Spain"),
        RegionsOfWine::Chile => println!("You can NOT produce wine in Chile"),
        RegionsOfWine::Argentina => println!("You can produce wine in Argentina"),
        RegionsOfWine::USA => println!("You can produce wine in USA"),
        RegionsOfWine::Australia => println!("You can produce wine in Australia"),
        RegionsOfWine::SouthAfrica => println!("You can produce wine in South Africa"),
    }
}

fn popularity_region(region: RegionsOfWine) {
    match region {
        RegionsOfWine::France => println!("France is the most popular region"),
        RegionsOfWine::Italy => println!("Italy is the most popular region"),
        RegionsOfWine::Spain => println!("Spain is the most popular region"),
        RegionsOfWine::Chile => println!("Chile is the most popular region"),
        RegionsOfWine::Argentina => println!("Argentina is the most popular region"),
        RegionsOfWine::USA => println!("USA is the most popular region"),
        RegionsOfWine::Australia => println!("Australia is the most popular region"),
        RegionsOfWine::SouthAfrica => println!("South Africa is the most popular region"),
    }
}