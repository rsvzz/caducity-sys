use gtk::glib;
use glib::{Object, Properties, DateTime, prelude::*, subclass::prelude::*};
use std::cell::RefCell;

glib::wrapper! {
    pub struct ItemCategory(ObjectSubclass<imp::ItemCategory>);
}

impl ItemCategory {
    pub fn new(
        id: u32,
        name: String,
        description: String,
        date_created: Option<DateTime>,
        date_updated: Option<DateTime>,
    ) -> Self {
        Object::builder()
            .property("id", id)
            .property("name", name)
            .property("description", description)
            .property("date_created", date_created)
            .property("date_updated", date_updated)
            .build()
    }
}

// Default value Empty
impl Default for ItemCategory {
    fn default() -> Self {
        Object::builder()
            .property("id", 0)
            .property("name", "")
            .property("description", "")
            .property("date_created", None::<DateTime>)
            .property("date_updated", None::<DateTime>)
            .build()
    }
}

mod imp {

    use super::*;

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::ItemCategory)]
    pub struct ItemCategory {
        #[property(get, set)]
        id: RefCell<u32>,
        #[property(get, set)]
        name: RefCell<String>,
        #[property(get, set)]
        description: RefCell<String>,
        #[property(get, set)]
        date_created: RefCell<Option<DateTime>>,
        #[property(get, set)]
        date_updated: RefCell<Option<DateTime>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ItemCategory {
        const NAME: &'static str = "ItemCategory";
        type Type = super::ItemCategory;
        type ParentType = glib::Object;
    }

    #[glib::derived_properties]
    impl ObjectImpl for ItemCategory {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }
}
