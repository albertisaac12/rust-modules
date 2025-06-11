pub mod inventory;
pub mod orders;

pub use std::{fmt,io::{self,stdin,stdout},collections::*};

pub use crate::inventory::{products, Item, FLOOR_SPACE as fp};