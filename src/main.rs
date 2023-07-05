mod to_do;
use to_do::traits::delete::Delete;
use to_do::traits::edit::Edit;
use to_do::traits::get::Get;

use crate::to_do::enums::TaskStatus;
use crate::to_do::{to_do_factory, ItemTypes};

fn main() {
    let to_do_item = to_do_factory("washing", TaskStatus::Done);

    match to_do_item {
        ItemTypes::Done(item) => {
            item.get(&item.super_struct.title);
            item.delete(&item.super_struct.title);
        }
        ItemTypes::Pending(item) => {
            item.get(&item.super_struct.title);
            item.set_to_done(&item.super_struct.title);
        }
    }
}
