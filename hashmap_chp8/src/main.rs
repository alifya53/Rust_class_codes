use std::collections::HashMap;
fn main() {
// let mut product = HashMap::new();
let mut menu = HashMap::with_capacity(5);
println!("Capacity = {}",menu.capacity());
menu.insert(String::from("Pakistani"),1);
menu.insert(String::from("Fast Food"),2);
menu.insert(String::from("B.B.Q"),3);
println!("{:?}",menu);

// product.insert(String::from("Chicken Burger"),250);
// product.insert(String::from("Zinger Burger"),350);

// println!("{:?}",product);
    
}
