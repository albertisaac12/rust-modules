mod inventory;
mod orders;

use crate::inventory::{FLOOR_SPACE as fp};
fn main() {
  
  println!("The manager of our inventory is {} ", crate::inventory::MANAGER); 
  println!("The manager of our orders is {} ", crate::orders::MANAGER); 


  println!("Our mangers are {} and {} , we have this much amount of floor space {} in sq feet", crate::inventory::MANAGER,crate::orders::MANAGER, fp);

  let fav_cat =  crate::inventory::products::ProductCategory::Hammer;
  println!("My fav cat of item is {fav_cat:?}");

  let tall_ladder = crate::inventory::products::Item {
    name: "ladder".to_string(),
    category: crate::inventory::products::ProductCategory::Ladder,
    quantity: 100,
  };


}

// Packages and Crates

// Cargo will look for a src/main.rs file. If it exists, Rust infers that we have a binary crate named mods. (main.rs => same as src/mods.rs (mods is the package name))
// Cargo will look for a src/lib.rs or lib/mods.rs file. If it exists, Rust infers that we have a library crate named mods. (lib.rs => same as lib/mods.rs (mods is the package name)) // need to change

// A module is an organizational container that encapsulates related code. 

// :: is a scope resolution operator

// the pub keyword makes sure that the element is accessible outside its namespace (scope).

// create root is the base/foundation of a crate (the starting point for the compiler)

//file for module `orders` found at both "src/orders.rs" and "src/orders/mod.rs" this will be an error we can either have only one orders.rs or one orders/mod.rs

// the struct felids are private by default too, we mark the methods inside the impl pub if needed