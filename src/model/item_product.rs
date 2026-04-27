use glib::prelude::*;
use glib::{DateTime, Object, Properties, subclass::prelude::*};
use gtk::glib;
use std::cell::RefCell;

glib::wrapper! {
    pub struct ItemProduct(ObjectSubclass<imp::ItemProduct>);
}

impl ItemProduct {
    pub fn new(
        barcode: String,
        name: String,
        description: String,
        id_category: u32,
        date_created: Option<DateTime>,
        date_updated: Option<DateTime>,
    ) -> Self {
        Object::builder()
            .property("barcode", barcode)
            .property("name", name)
            .property("description", description)
            .property("category_id", id_category)
            .property("is_created", false)
            .property("date_created", date_created)
            .property("date_updated", date_updated)
            .build()
    }
}

// Default value Empty
impl Default for ItemProduct {
    fn default() -> Self {
        Object::builder()
            .property("barcode", "")
            .property("name", "")
            .property("description", "")
            .property("category_id", 0)
            .property("is_created", false)
            .property("date_created", None::<DateTime>)
            .property("date_updated", None::<DateTime>)
            .build()
    }
}

mod imp {
    use super::*;
    ///ItemProduct is GObject for ColumnView
    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::ItemProduct)]
    pub struct ItemProduct {
        #[property(get, set)]
        barcode: RefCell<String>,
        #[property(get, set)]
        name: RefCell<String>,
        #[property(get, set)]
        description: RefCell<String>,
        ///if data db true exist false not exist
        #[property(get, set)]
        is_created: RefCell<bool>,
        ///id category need foreign key
        #[property(get, set)]
        category_id: RefCell<u32>,
        #[property(get, set)]
        date_created: RefCell<Option<DateTime>>,
        #[property(get, set)]
        date_updated: RefCell<Option<DateTime>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ItemProduct {
        const NAME: &'static str = "ItemProduct";
        type Type = super::ItemProduct;
        type ParentType = glib::Object;
    }

    #[glib::derived_properties]
    impl ObjectImpl for ItemProduct {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }
}
