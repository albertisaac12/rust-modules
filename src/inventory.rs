pub mod products;

pub use products::{Item,ProductCategory}; // will export the Item and ProductCategory out of the inventory module directly even if they belong to a different module and have been imported to use.

pub const FLOOR_SPACE: i32 = 10000;
pub const MANAGER: &str = "Ivan Inventory";

fn talk_to_manager() {
    println!("Hey {MANAGER}, how is your coffee ?, What do you think of {:?}", ProductCategory::Ladder);
}

    // pub mod products {
    //      #[derive(Debug)]
    //     pub enum ProductCategory {
    //         Ladder,
    //         Hammer
    //     }
    
    //     #[derive(Debug)]
    //     pub struct Item {
    //         pub name: String,
    //         pub category: ProductCategory,
    //         pub quantity: u32
    //     }

    // }


