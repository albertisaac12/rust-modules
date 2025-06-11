use fake::{Faker,Fake};
use warehouse::{products::ProductCategory};
fn main() {
    println!("It's a.............. {:?}", ProductCategory::Hammer);
    
    let pc: ProductCategory = Faker.fake();
    
    println!("It's a.............. {:?}",pc);
    
}