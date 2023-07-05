use crate::to_do::{
    enums::TaskStatus,
    traits::{create::Create, edit::Edit, get::Get},
};

use super::base::Base;

pub struct Pending {
    pub super_struct: Base,
}

impl Pending {
    pub fn new(input_title: &str) -> Self {
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::Pending,
        };

        Pending { super_struct: base }
    }
}

impl Get for Pending {}
impl Create for Pending {}
impl Edit for Pending {}
