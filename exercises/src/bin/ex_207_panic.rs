
// FILL the blanks
fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Success!");
        // IMPLEMENT the below code
        panic!("call panic when beverage is lemonade")
     }

    println!("Exercise Failed if printing out this line!");
}

fn main() {
    drink("lemonade");

    println!("Exercise Failed if printing out this line!");
}