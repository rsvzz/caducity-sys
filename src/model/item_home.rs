use glib::prelude::*;
use glib::{Object, Properties, subclass::prelude::*};
use gtk::glib::{self, DateTime};
use std::cell::RefCell;

glib::wrapper! {
    pub struct ItemHome(ObjectSubclass<imp::ItemHome>);
}

impl ItemHome {
    pub fn new(
        id: u32,
        status: u8,
        date_created: Option<DateTime>,
        date_updated: Option<DateTime>,
    ) -> Self {
        Object::builder()
            .property("id", id)
            .property("statusid", status)
            .property("date_created", date_created)
            .property("date_updated", date_updated)
            .build()
    }
}

// Default value Empty
impl Default for ItemHome {
    fn default() -> Self {
        Object::builder()
            .property("id", 0)
            .property("statusid", 0)
            .property("date_created", None::<DateTime>)
            .property("date_updated", None::<DateTime>)
            .build()
    }
}

mod imp {
    use super::*;
    ///Inventory Item for view in ColumnView
    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::ItemHome)]
    pub struct ItemHome {
        #[property(get, set)]
        id: RefCell<u32>,
        #[property(get, set)]
        statusid: RefCell<u8>,
        ///Date for Glib
        #[property(get, set)]
        date_created: RefCell<Option<DateTime>>,
        ///Date for Glib
        #[property(get, set)]
        date_updated: RefCell<Option<DateTime>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ItemHome {
        const NAME: &'static str = "ItemHome";
        type Type = super::ItemHome;
        type ParentType = glib::Object;
    }

    #[glib::derived_properties]
    impl ObjectImpl for ItemHome {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }
}
