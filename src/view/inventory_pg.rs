use crate::data::InventoryDetLite;
use crate::model::{
    CreateLiteExt, DataConext, ItemInventory, LoadUI, QueryFilterContext,
};
use crate::view::{DialogAlert, UpdateViewExt};
use adw::{ApplicationWindow, ComboRow, prelude::*};
use adw::{EntryRow, ToolbarView};
use chrono::{Datelike, NaiveDate};
use gtk::gio::ListStore;
use gtk::glib::{self, Date, DateDay, DateMonth, DateTime, DateYear, TimeZone};
use gtk::{
    Builder, Button, Calendar, ColumnView, ColumnViewColumn, Label, Revealer,
    SignalListItemFactory, SingleSelection,
};
use rsqlite::Connection;

#[derive(Clone)]
pub struct InventoryPG {
    tool_view: ToolbarView,
    db_path: String,
    id_inventory: u32,
    column_view: ColumnView,
    et_id: EntryRow,
    et_inv_id: EntryRow,
    et_barcode: EntryRow,
    et_stock: EntryRow,
    btn_delete: Button,
    rev_add: Revealer,
    btn_save: Button,
    /// status inventory products
    cb_status: ComboRow,
    /// type box
    cb_box: ComboRow,
    /// Date Caducity Product
    date_caducity: Calendar,
    parent: ApplicationWindow,
}

impl InventoryPG {
    pub fn new(ui: &LoadUI, db: &str, id_inv: u32, parent: &ApplicationWindow) -> Self {
        let _build: Builder = ui.get_new_ui().build; //new builder
        let _tool_view: ToolbarView = _build.object("tool_view").unwrap();
        let btn_add: Button = _build.object("btn_show_add").unwrap();
        let column_view: ColumnView = _build.object("view_list").unwrap();
        let _rev_add: Revealer = _build.object("rev_add").unwrap();
        let _et_id: EntryRow = _build.object("et_id").unwrap();
        let et_inv_id: EntryRow = _build.object("et_inv_id").unwrap();
        let et_barcode: EntryRow = _build.object("et_barcode").unwrap();
        let et_stock: EntryRow = _build.object("et_stock").unwrap();
        let cb_status: ComboRow = _build.object("cb_status").unwrap();
        let cb_box: ComboRow = _build.object("cb_box").unwrap();
        let c_date_caducity: Calendar = _build.object("c_date_caducity").unwrap();
        let _parent = parent.clone();
        let lbl_title: Label = _build.object("lbl_title").unwrap();

        let btn_cancel: Button = _build.object("btn_cancel").unwrap();

        let _btn_save: Button = _build.object("btn_save").unwrap();
        let btn_edit: Button = _build.object("btn_edit").unwrap();
        let btn_delete: Button = _build.object("btn_delete").unwrap();

        lbl_title.set_text(&format!("Datail Inventory # {}", id_inv).to_string());

        let _build_inventory = _build.clone();
        let id_inv_cp = id_inv;
        btn_add.connect_clicked(glib::clone!(
            #[weak]
            _rev_add,
            #[weak]
            et_inv_id,
            #[weak]
            et_barcode,
            move |_| {
                _rev_add.set_reveal_child(!_rev_add.reveals_child());
                et_inv_id.set_text(&id_inv_cp.to_string());
                et_barcode.grab_focus();
            }
        ));

        btn_cancel.connect_clicked(glib::clone!(
            #[weak]
            _rev_add,
            #[weak]
            et_barcode,
            #[weak]
            _et_id,
            #[weak]
            et_inv_id,
            #[weak]
            et_stock,
            #[weak]
            cb_status,
            #[weak]
            cb_box,
            #[weak]
            c_date_caducity,
            move |_| {
                _rev_add.set_reveal_child(false);
                _et_id.set_text("");
                et_inv_id.set_text("");
                et_barcode.set_text("");
                et_stock.set_text("");
                cb_status.set_selected(0); //set default selection
                cb_box.set_selected(0); //set default selection

                if let Ok(date) = DateTime::now_local() {
                    //let today = c_date_caducity.date();
                    c_date_caducity.select_day(&date); //day now
                }
            }
        ));

        btn_edit.connect_clicked(glib::clone!(
            #[weak]
            column_view,
            #[weak]
            _rev_add,
            #[weak]
            cb_status,
            #[weak]
            et_barcode,
            #[weak]
            cb_box,
            #[weak]
            c_date_caducity,
            #[weak]
            et_stock,
            #[weak]
            _et_id,
            #[weak]
            et_inv_id,
            move |_| {
                if !_rev_add.is_child_revealed() {
                    if let Some(select) = column_view.model().and_downcast::<SingleSelection>() {
                        //select item ColumnView
                        if let Some(obj) = select.item(select.selected()) {
                            //ItemInventory
                            if let Ok(item) = obj.downcast::<ItemInventory>() {
                                if let Some(model) = cb_box.model() {
                                    for idex in 0..model.n_items() {
                                        if let Some(_) = model.item(idex) {
                                            if item.box_id() == idex as u8 {
                                                cb_box.set_selected(idex);
                                                break;
                                            }
                                        }
                                    }
                                }

                                if let Some(model) = cb_status.model() {
                                    for idex in 0..model.n_items() {
                                        if let Some(_) = model.item(idex) {
                                            if item.status_id() == idex as u8 {
                                                cb_status.set_selected(idex);
                                                break;
                                            }
                                        }
                                    }
                                }
                                //id_inv.set_text();
                                et_barcode.set_text(&item.barcode().to_string());
                                et_barcode.set_sensitive(false);
                                et_stock.set_text(&item.stock().to_string());
                                _et_id.set_text(&item.id_inv_det().to_string());
                                et_inv_id.set_text(&item.id_inv().to_string());

                                if let Some(date) = item.date_caducity() {
                                    let month = match date.month() {
                                        DateMonth::January => 1,
                                        DateMonth::February => 2,
                                        DateMonth::March => 3,
                                        DateMonth::April => 4,
                                        DateMonth::May => 5,
                                        DateMonth::June => 6,
                                        DateMonth::July => 7,
                                        DateMonth::August => 8,
                                        DateMonth::September => 9,
                                        DateMonth::October => 10,
                                        DateMonth::November => 11,
                                        DateMonth::December => 12,
                                        DateMonth::BadMonth => 0,
                                        _ => 0,
                                    };
                                    let tz = TimeZone::utc();

                                    let datetime = DateTime::new(
                                        &tz,
                                        date.year() as i32,
                                        month,
                                        date.day() as i32,
                                        0,
                                        0,
                                        0.0,
                                    )
                                    .unwrap();

                                    c_date_caducity.select_day(&datetime);
                                }
                                _rev_add.set_reveal_child(true);
                            }
                        }
                    }
                }
            }
        ));

        InventoryPG {
            tool_view: _tool_view,
            db_path: db.to_string(),
            id_inventory: id_inv,
            column_view: column_view,
            et_id: _et_id,
            et_inv_id: et_inv_id,
            et_barcode: et_barcode,
            et_stock: et_stock,
            btn_delete: btn_delete,
            rev_add: _rev_add,
            btn_save: _btn_save,
            cb_status: cb_status,
            cb_box: cb_box,
            date_caducity: c_date_caducity,
            parent: _parent,
        }
    }

    pub fn get_tool_view(&self) -> &ToolbarView {
        &self.tool_view
    }
}

impl UpdateViewExt for InventoryPG {
    fn connect_signal_view(&self) {
        let et_barcode = self.et_barcode.clone();
        let et_inv_id = self.et_inv_id.clone();
        let revealer = self.rev_add.clone();
        let et_id = self.et_id.clone();
        let cb_status = self.cb_status.clone();
        let et_stock = self.et_stock.clone();
        let cb_box = self.cb_box.clone();
        let db_str = self.db_path.to_string();
        let c_caducity = self.date_caducity.clone();
        let win_cp = self.parent.clone();
        let myobj = self.clone();
        let column_view = self.column_view.clone();

        let _path_db_cp = db_str.to_string();
        let _myobj_cp = myobj.clone();
        let win_cp_del = win_cp.clone();
        self.btn_delete.connect_clicked(glib::clone!(
            #[weak]
            et_id,
            #[weak]
            column_view,
            #[weak]
            revealer,
            move |_| {
                if !revealer.is_child_revealed() {
                    if let Some(select) = column_view.model().and_downcast::<SingleSelection>() {
                        //select item ColumnView
                        if let Some(obj) = select.item(select.selected()) {
                            //ItemInventory
                            if let Ok(item) = obj.downcast::<ItemInventory>() {
                                let alert = DialogAlert::new(
                                    "Do you want to do this?",
                                    "If you delete data",
                                );
                                let _id = match et_id.text().parse::<u32>() {
                                    Ok(num) => num,
                                    Err(_) => 0,
                                };

                                let _obj_inv = InventoryDetLite::del(item.id_inv_det());
                                let path = _path_db_cp.to_string();
                                let _obj_cp = _myobj_cp.clone();
                                alert.dialog.connect_response(
                                    None,
                                    move |_, str: &str| match str {
                                        "accept" => {
                                            let ctx: &dyn DataConext<
                                                Output = Connection,
                                                Error = rsqlite::Error,
                                            > = &_obj_inv;
                                            match ctx.delete(&path) {
                                                Ok(_) => _obj_cp.refresh(),
                                                Err(e) => println!("{}", e),
                                            }
                                        }
                                        "cancel" => {}
                                        _ => {}
                                    },
                                );
                                alert.dialog.present(Some(&win_cp_del));
                            }
                        }
                    }
                }
            }
        ));

        let _myobj_cp2 = myobj.clone();
        let _db_str_cp = db_str.to_string();
        self.btn_save.connect_clicked(glib::clone!(
            #[weak]
            et_barcode,
            #[weak]
            et_inv_id,
            #[weak]
            et_id,
            #[weak]
            c_caducity,
            #[weak]
            cb_box,
            #[weak]
            cb_status,
            #[weak]
            revealer,
            move |_| {
                let alert = DialogAlert::new("Do you want to do this?", "If you create data");
                let _id_inv = match et_inv_id.text().parse::<u32>() {
                    Ok(num) => num,
                    Err(_) => 0,
                };

                let _box_id = cb_box.selected() as u8;
                let _stock = match et_stock.text().parse::<u32>() {
                    Ok(value) => value,
                    Err(_) => 0,
                };

                let _status_id = cb_status.selected() as u8;
                let _caducity = c_caducity.date();
                let c_date = NaiveDate::from_ymd_opt(
                    _caducity.year(),
                    _caducity.month() as u32,
                    _caducity.day_of_month() as u32,
                );

                let _db_cp = _db_str_cp.to_string();
                let _obj_cp = myobj.clone();
                let mut _obj_inv = InventoryDetLite::new(
                    _id_inv,
                    et_barcode.text().to_string(),
                    _stock,
                    _status_id,
                    _box_id,
                    c_date,
                );

                if et_id.text().to_string().len() == 0 {
                    let _et_id = et_id.clone();
                    let _et_id_inv = et_inv_id.clone();
                    let _et_stock = et_stock.clone();
                    let _barcode = et_barcode.clone();
                    let _caducity = c_caducity.clone();
                    let _cbox = cb_box.clone();
                    let _cstatus = cb_status.clone();

                    alert.dialog.connect_response(None, move |_, str: &str| {
                                        match str {
                                            "accept" => {
                                                let ctx: &dyn DataConext<
                                                    Output = Connection,
                                                    Error = rsqlite::Error,
                                                > = &_obj_inv;
                                                match ctx.insert(&_db_cp) {
                                                    Ok(_) => {
                                                        _et_id.set_text("");
                                                        _et_id_inv.set_text("");
                                                        _et_stock.set_text("");
                                                        _barcode.set_text("");
                                                        _cbox.set_selected(0);
                                                        _cstatus.set_selected(0);

                                                         if let Ok(date) = DateTime::now_local() {
                                                            _caducity.select_day(&date); //day now
                                                        }

                                                        _obj_cp.refresh();
                                                    },
                                                    Err(e) => println!("{}", e),
                                                }
                                            }
                                            "cancel" => {}
                                            _ => {}
                                        }
                                    });
                } else {
                    alert.set_body("If you update data");
                    let _db_cp = db_str.to_string();
                    let _obj_cp = myobj.clone();
                    _obj_inv.id_ivt_det = et_id.text().parse::<u32>().unwrap();

                    let _et_id = et_id.clone();
                    let _et_id_inv = et_inv_id.clone();
                    let _barcode = et_barcode.clone();
                    let _et_stock = et_stock.clone();
                    let _caducity = c_caducity.clone();
                    let _cbox = cb_box.clone();
                    let _cstatus = cb_status.clone();
                    let _revealer = revealer.clone();
                    let _obj_cp = _myobj_cp2.clone();
                    alert.dialog.connect_response(None, move |_, str: &str| {
                                        match str {
                                            "accept" => {
                                                let ctx: &dyn DataConext<
                                                    Output = Connection,
                                                    Error = rsqlite::Error,
                                                > = &_obj_inv;
                                                match ctx.update(&_db_cp) {
                                                    Ok(_) => {
                                                        _et_id.set_text("");
                                                        _et_id_inv.set_text("");
                                                        _et_stock.set_text("");
                                                        _barcode.set_text("");
                                                        _cbox.set_selected(0);
                                                        _cstatus.set_selected(0);
                                                        if let Ok(date) = DateTime::now_local() {
                                                            _caducity.select_day(&date); //day now
                                                        }
                                                        _revealer.set_reveal_child(false);
                                                        _obj_cp.refresh();

                                                    },
                                                    Err(e) => println!("{}", e),
                                                }
                                            }
                                            "cancel" => {}
                                            _ => {}
                                        }
                                    });
                }

                alert.dialog.present(Some(&win_cp));
            }
        ));
    }

    fn refresh(&self) {
        let store = ListStore::builder()
            .item_type(ItemInventory::static_type())
            .build();

        let mut obj_inv = InventoryDetLite::empty();
        obj_inv.inv_id = self.id_inventory; // filter por id_inv Id Inventory
        let query: &dyn QueryFilterContext<Output = Option<Vec<InventoryDetLite>>> = &obj_inv;

        if let Some(list) = query.get_filter(self.db_path.as_str()) {
            for item in list.iter() {
                let date_created: Option<DateTime>;
                let date_updated: Option<DateTime>;
                let date_exps: Option<Date>;

                if let Some(date_upd) = item.date_updated {
                    date_updated = Some(
                        DateTime::from_unix_utc(date_upd.and_utc().timestamp()).expect("REASON"),
                    );
                } else {
                    date_updated = None::<DateTime>;
                }

                if let Some(date_cre) = item.date_created {
                    date_created = Some(
                        DateTime::from_unix_utc(date_cre.and_utc().timestamp()).expect("REASON"),
                    );
                } else {
                    date_created = None::<DateTime>;
                }

                if let Some(date_ex) = item.date_expiration {
                    date_exps = Some(
                        Date::from_dmy(
                            DateDay::from(date_ex.day() as u8),
                            match date_ex.month() {
                                1 => DateMonth::January,
                                2 => DateMonth::February,
                                3 => DateMonth::March,
                                4 => DateMonth::April,
                                5 => DateMonth::May,
                                6 => DateMonth::June,
                                7 => DateMonth::July,
                                8 => DateMonth::August,
                                9 => DateMonth::September,
                                10 => DateMonth::October,
                                11 => DateMonth::November,
                                12 => DateMonth::December,
                                _ => DateMonth::BadMonth,
                            },
                            DateYear::from(date_ex.year() as u16),
                        )
                        .expect("REASON"),
                    );
                } else {
                    date_exps = None::<Date>;
                }

                let product = ItemInventory::new(
                    item.id_ivt_det,
                    item.inv_id,
                    item.barcode.to_string(),
                    item.stock,
                    item.status_id,
                    item.box_id,
                    date_exps,
                    date_created,
                    date_updated,
                );

                store.append(&product);
            }
        }

        let select = SingleSelection::new(Some(store));
        self.column_view.set_model(Some(&select));

        let factory_id = SignalListItemFactory::new();
        factory_id.connect_setup(|_, obj| {
            let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
            let label = Label::new(Some(""));
            list_item.set_child(Some(&label));
        });

        factory_id.connect_bind(|_, obj| {
            let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
            let label = list_item.child().unwrap().downcast::<gtk::Label>().unwrap();
            let item = list_item.item().and_downcast::<ItemInventory>().unwrap();
            label.set_text(&item.id_inv_det().to_string());
        });

        let factory_id_inv = SignalListItemFactory::new();
        factory_id_inv.connect_setup(|_, obj| {
            let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
            let label = Label::new(Some(""));
            list_item.set_child(Some(&label));
        });

        factory_id_inv.connect_bind(|_, obj| {
            let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
            let label = list_item.child().unwrap().downcast::<gtk::Label>().unwrap();
            let item = list_item.item().and_downcast::<ItemInventory>().unwrap();
            label.set_text(&item.id_inv().to_string());
        });

        let factory_barcode = SignalListItemFactory::new();
        factory_barcode.connect_setup(|_, obj| {
            let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
            let label = Label::new(Some(""));
            list_item.set_child(Some(&label));
        });
        factory_barcode.connect_bind(|_, obj| {
            let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
            let label = list_item.child().unwrap().downcast::<gtk::Label>().unwrap();
            let item = list_item.item().and_downcast::<ItemInventory>().unwrap();
            label.set_text(&item.barcode());
        });

        let factory_stock = SignalListItemFactory::new();
        factory_stock.connect_setup(|_, obj| {
            let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
            let label = Label::new(Some(""));
            list_item.set_child(Some(&label));
        });
        factory_stock.connect_bind(|_, obj| {
            let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
            let label = list_item.child().unwrap().downcast::<gtk::Label>().unwrap();
            let item = list_item.item().and_downcast::<ItemInventory>().unwrap();
            label.set_text(&item.stock().to_string());
        });

        let factory_box = SignalListItemFactory::new();
        factory_box.connect_setup(|_, obj| {
            let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
            let label = Label::new(Some(""));
            list_item.set_child(Some(&label));
        });

        factory_box.connect_bind(|_, obj| {
            let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
            let label = list_item.child().unwrap().downcast::<gtk::Label>().unwrap();
            let item = list_item.item().and_downcast::<ItemInventory>().unwrap();
            label.set_text(&item.box_id().to_string());
        });

        let factory_status = SignalListItemFactory::new();
        factory_status.connect_setup(|_, obj| {
            let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
            let label = Label::new(Some(""));
            list_item.set_child(Some(&label));
        });

        factory_status.connect_bind(|_, obj| {
            let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
            let label = list_item.child().unwrap().downcast::<gtk::Label>().unwrap();
            let item = list_item.item().and_downcast::<ItemInventory>().unwrap();
            label.set_text(&item.status_id().to_string());
        });

        let factory_date = SignalListItemFactory::new();
        factory_date.connect_setup(|_, obj| {
            let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
            let label = Label::new(Some(""));
            list_item.set_child(Some(&label));
        });

        factory_date.connect_bind(|_, obj| {
            let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
            let label = list_item.child().unwrap().downcast::<gtk::Label>().unwrap();
            let item = list_item.item().and_downcast::<ItemInventory>().unwrap();
            if let Some(date) = item.date_caducity() {
                let month = match date.month() {
                    DateMonth::January => 1,
                    DateMonth::February => 2,
                    DateMonth::March => 3,
                    DateMonth::April => 4,
                    DateMonth::May => 5,
                    DateMonth::June => 6,
                    DateMonth::July => 7,
                    DateMonth::August => 8,
                    DateMonth::September => 9,
                    DateMonth::October => 10,
                    DateMonth::November => 11,
                    DateMonth::December => 12,
                    DateMonth::BadMonth => 0,
                    _ => 0,
                };
                let tz = TimeZone::utc();

                let datetime =
                    DateTime::new(&tz, date.year() as i32, month, date.day() as i32, 0, 0, 0.0)
                        .unwrap();
                label.set_text(&datetime.format("%Y-%m-%d").unwrap());
            }
        });

        let factory_created: SignalListItemFactory = SignalListItemFactory::new();
        factory_created.connect_setup(|_, obj| {
            let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
            let label = Label::new(Some(""));
            list_item.set_child(Some(&label));
        });
        factory_created.connect_bind(|_, obj| {
            let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
            let label = list_item.child().unwrap().downcast::<gtk::Label>().unwrap();
            let item = list_item.item().and_downcast::<ItemInventory>().unwrap();
            if let Some(date_created) = item.date_created() {
                label.set_text(&date_created.format("%Y-%m-%d %H:%M:%S").unwrap());
            }
        });

        let factory_updated = SignalListItemFactory::new();
        factory_updated.connect_setup(|_, obj| {
            let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
            let label = Label::new(Some(""));
            list_item.set_child(Some(&label));
        });
        factory_updated.connect_bind(|_, obj| {
            let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
            let label = list_item.child().unwrap().downcast::<gtk::Label>().unwrap();
            let item = list_item.item().and_downcast::<ItemInventory>().unwrap();
            if let Some(date_updated) = item.date_updated() {
                label.set_text(&date_updated.format("%Y-%m-%d %H:%M:%S").unwrap());
            }
        });

        let col_id = ColumnViewColumn::new(Some("Id"), Some(factory_id));
        let col_id_inv = ColumnViewColumn::new(Some("Inventory"), Some(factory_id_inv));
        let col_barcode = ColumnViewColumn::new(Some("Barcode"), Some(factory_barcode));
        let col_stock = ColumnViewColumn::new(Some("Stock"), Some(factory_stock));
        let col_status = ColumnViewColumn::new(Some("Status"), Some(factory_status));
        let col_box = ColumnViewColumn::new(Some("Boxes"), Some(factory_box));
        let col_date = ColumnViewColumn::new(Some("Date"), Some(factory_date));
        let col_date_created = ColumnViewColumn::new(Some("Date Created"), Some(factory_created));
        let col_date_updated = ColumnViewColumn::new(Some("Date Updated"), Some(factory_updated));

        col_barcode.set_fixed_width(80);
        col_barcode.set_resizable(true);
        col_barcode.set_expand(true);
        col_stock.set_fixed_width(60);
        col_stock.set_resizable(true);
        col_stock.set_expand(true);
        col_box.set_fixed_width(80);
        col_box.set_resizable(true);
        col_box.set_expand(true);

        col_date.set_fixed_width(80);
        col_date.set_resizable(true);
        col_date.set_expand(true);

        let mut item_col = Vec::new();
        for idex in 0..self.column_view.columns().n_items() {
            if let Some(item) = self
                .column_view
                .columns()
                .item(idex)
                .and_downcast::<ColumnViewColumn>()
            {
                item_col.push(item);
            }
        }

        for col in item_col {
            self.column_view.remove_column(&col);
        }

        self.column_view.append_column(&col_id);
        self.column_view.append_column(&col_id_inv);
        self.column_view.append_column(&col_barcode);
        self.column_view.append_column(&col_stock);
        self.column_view.append_column(&col_status);
        self.column_view.append_column(&col_box);
        self.column_view.append_column(&col_date);
        self.column_view.append_column(&col_date_created);
        self.column_view.append_column(&col_date_updated);
    }
}
