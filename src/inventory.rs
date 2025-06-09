
    pub const FLOOR_SPACE: i32 = 10000;
    pub const MANAGER: &str = "Ivan Inventory";

    fn talk_to_manager() {
        println!("Hey {MANAGER}, how is your coffee ?");
    }

    pub mod products;
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


