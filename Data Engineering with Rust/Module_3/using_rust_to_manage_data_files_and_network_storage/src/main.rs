//from official documentation
// use aws_sdk_dynamodb::{Client, Error};

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     let shared_config = aws_config::load_from_env().await;
//     let client = Client::new(&shared_config);
//     let req = client.list_tables().limit(10);
//     let resp = req.send().await?;
//     println!("Current DynamoDB tables: {:?}", resp.table_names);
//     Ok(())
// }

use csv::Reader;
use csv::Writer;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a list of fruits and their prices
    let fruits = [
        ("Apple", 1.25),
        ("Banana", 0.75),
        ("Orange", 1.00),
        ("Mango", 2.50),
        ("Pineapple", 3.00),
    ];

    // Open the CSV file in write mode
    let mut wtr = Writer::from_path("output.csv")?;
    let mut _read = Reader::from_path("output.csv")?;

    // Write the header row
    wtr.write_record(["Fruit", "Price"])?;

    // Write each fruit and its price to the CSV file
    for (fruit, price) in fruits {
        wtr.write_record([fruit, &price.to_string()])?; // Convert price to string
    }

    wtr.flush()?; // Ensure data is written

    // Iterate through the rows of output.csv file and extract the product name and price
    for result in _read.records() {
        let record = result?;
        let fruit = &record[0];
        let price = &record[1];
        println!(
            "Fruit: {}, Price: {}, Discount: {}",
            fruit,
            price,
            price.parse::<f64>().unwrap() * 0.1
        );
    }

    let mut products = Writer::from_path("discounted_products.csv")?;

    products.write_record(["Fruit", "Price", "Discounted Price"])?;

    let mut _read = Reader::from_path("output.csv")?; // Re-create the Reader instance

    for result in _read.records() {
        let record = result?;
        let fruit = &record[0];
        let price = &record[1];
        let discount = price.parse::<f64>().unwrap() * 0.1;
        let discounted_price = price.parse::<f64>().unwrap() - discount;
        products.write_record([fruit, price, &discounted_price.to_string()])?;
    }

    products.flush()?;

    Ok(())
}
