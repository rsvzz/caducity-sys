use glib::Date;
use glib::prelude::*;
use glib::{Object, Properties, subclass::prelude::*};
use gtk::glib;
use std::cell::RefCell;

use gtk::glib::DateTime;

glib::wrapper! {
    pub struct ItemInventory(ObjectSubclass<imp::ItemInventory>);
}

impl ItemInventory {
    pub fn new(id: u32, id_inv: u32, barcode: String, stock: u32, status_id: u8, box_id: u8, date: Option<Date>, d_created: Option<DateTime>, d_updated:Option<DateTime>) -> Self {
        Object::builder()
            .property("id_inv_det", id)
            .property("id_inv", id_inv)
            .property("barcode", barcode)
            .property("status_id", status_id)
            .property("box_id", box_id)
            .property("stock", stock)
            .property("date_caducity", date)
            .property("date_created", d_created)
            .property("date_updated", d_updated)
            .build()
    }
}

// Default value Empty
impl Default for ItemInventory {
    fn default() -> Self {
        Object::builder()
            .property("id_inv_det", 0)
            .property("id_inv", 0)
            .property("barcode", "")
            .property("status_id", 0)
            .property("box_id", 0)
            .property("stock", 0)
            .property("date_caducity", None::<Date>)
            .property("date_created", None::<DateTime>)
            .property("date_updated", None::<DateTime>)
            .build()
    }
}

mod imp {
    use super::*;

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::ItemInventory)]
    pub struct ItemInventory {
        #[property(get, set)]
        id_inv_det: RefCell<u32>,
        #[property(get, set)]
        id_inv: RefCell<u32>,
        #[property(get, set)]
        barcode: RefCell<String>,
        #[property(get, set)]
        status_id: RefCell<u8>,
        #[property(get, set)]
        stock: RefCell<u32>,
        #[property(get, set)]
        box_id: RefCell<u8>,
        #[property(get, set)]
        date_caducity: RefCell<Option<Date>>,
        ///Date for Glib
        #[property(get, set)]
        date_created: RefCell<Option<DateTime>>,
        ///Date for Glib
        #[property(get, set)]
        date_updated: RefCell<Option<DateTime>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ItemInventory {
        const NAME: &'static str = "ItemInventory";
        type Type = super::ItemInventory;
        type ParentType = glib::Object;
    }

    #[glib::derived_properties]
    impl ObjectImpl for ItemInventory {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }
}
