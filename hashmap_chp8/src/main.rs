use std::collections::HashMap;
fn main() {
    // ways for declaring hashmap
// let mut menu = HashMap::with_capacity(5);
// println!("Capacity = {}",menu.capacity());
// menu.insert(String::from("Pakistani"),1);
// menu.insert(String::from("Fast Food"),2);
// menu.insert(String::from("B.B.Q"),3);
// println!("{:?}",menu);

// Creating a  new hashmap

let mut product = HashMap::new();
product.insert(String::from("Chicken Burger"),250);
product.insert(String::from("Zinger Burger"),350);
println!("{:?}",product);    

//creating a hashmap using collect method

let menu = vec![String::from("Fastfood"),String::from("B.B.Q"),String::from("Pakistani")];
let item = vec![String::from("Pizza"),String::from("Chicken Roll"),String::from("Biryani")];

let menu_list: HashMap<_,_> = menu.iter().zip(item.iter()).collect();
println!("{:?}",menu_list);

// Hashmap and ownership

let item_name = String::from("Pan Pizza");
let item_price = 500;
let mut rate_list = HashMap::new();
rate_list.insert(item_name,item_price);
println!("{:?}",rate_list);
// println!("{}",item_name); // value moved so can not access because string type stores on heap
println!("{}",item_price); // can be access here because integer datatype stores on stack memory

// Accessing value in Hashmap


}
