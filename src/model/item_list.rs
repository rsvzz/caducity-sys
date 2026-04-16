use gtk::glib;
use glib::prelude::*;
use glib::{Object, Properties, subclass::prelude::*};
use std::cell::RefCell;

glib::wrapper! {
    pub struct ItemList(ObjectSubclass<imp::ItemList>);
}

impl ItemList {

    pub fn new(name: String) -> Self {
        Object::builder()
            .property("name", name)
            .build()
    }
}

// Default value Empty
impl Default for ItemList {
      fn default() -> Self {
       Object::builder()
            .property("name", "")
            .build()
    }
}

mod imp {
    use super::*;

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::ItemList)]
    pub struct ItemList {
        #[property(get, set)]
        name: RefCell<String>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ItemList {
        const NAME: &'static str = "ItemList";
        type Type = super::ItemList;
        type ParentType = glib::Object;
    }

    #[glib::derived_properties]
    impl ObjectImpl for ItemList {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }
}