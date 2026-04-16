use adw::prelude::*;
use adw::{ApplicationWindow, ComboRow, EntryRow, ToolbarView};
use gtk::glib::{self, DateTime, Variant};
use gtk::{
    Box, Builder, Button, ColumnView, ColumnViewColumn, Label, Revealer, SignalListItemFactory,
    SingleSelection,
};
use rsqlite::Connection;

use crate::data::{CategoryLite, ProductLite};
use crate::model::{
    CreateLiteExt, DataConext, ItemCategory, ItemProduct, LoadUI, QueryContext, QueryFilterContext,
};

use crate::view::{DialogAlert, UpdateViewExt};

use gtk::gio::ListStore;
#[derive(Clone)]
pub struct ProductPG {
    tool_view: ToolbarView,
    db_str: String,
    btn_save: Button,
    btn_refresh: Button,
    btn_delete: Button,
    et_barcode: EntryRow,
    et_name: EntryRow,
    et_description: EntryRow,
    cb_category: ComboRow,
    rev_add: Revealer,
    column_view: ColumnView,
    path_db: String,
    parent: ApplicationWindow,
}

impl ProductPG {
    pub fn new(ui: &LoadUI, db: &str, win: &ApplicationWindow) -> Self {
        let _win = win.clone();
        let db_str = db.to_string();
        let _build: Builder = ui.build.clone();
        let _tool_view: ToolbarView = _build.object("tool_view").unwrap();
        let btn_add: Button = _build.object("btn_show_add").unwrap();
        let _btn_refresh: Button = _build.object("btn_refresh").unwrap();
        let btn_edit: Button = _build.object("btn_edit").unwrap();

        let btn_cancel: Button = _build.object("btn_cancel").unwrap();
        let btn_delete: Button = _build.object("btn_delete").unwrap();
        let rev_add: Revealer = _build.object("rev_add").unwrap();

        let column_view: ColumnView = _build.object("view_list").unwrap();
        let btn_save: Button = _build.object("btn_save").unwrap();
        btn_save.set_action_target_value(Some(&Variant::from(false)));
    
        let _et_barcode: EntryRow = _build.object("et_barcode").unwrap();
        let _et_name: EntryRow = _build.object("et_name").unwrap();
        let _et_description: EntryRow = _build.object("et_description").unwrap();
        let cb_category: ComboRow = _build.object("cb_category").unwrap();

        let _build_product = _build.clone();
        //let cate_cp = cate_lite.clone();
        let _db_path = db.to_string();

        btn_cancel.connect_clicked(glib::clone!(
            #[weak]
            rev_add,
            #[weak]
            _et_barcode,
            #[weak]
            _et_name,
            #[weak]
            _et_description,
            #[weak]
            btn_save,
            move |_| {
                rev_add.set_reveal_child(false);
                _et_barcode.set_text("");
                _et_name.set_text("");
                _et_description.set_text("");
                btn_save.set_action_target_value(Some(&Variant::from(false)));
            }
        ));

        btn_edit.connect_clicked(glib::clone!(
            #[weak]
            column_view,
            #[weak]
            rev_add,
            #[weak]
            cb_category,
            #[weak]
            _et_barcode,
            #[weak]
            _et_name,
            #[weak]
            _et_description,
            #[weak]
            btn_save,
            move |_| {
                if !rev_add.is_child_revealed() {
                    if let Some(select) = column_view.model().and_downcast::<SingleSelection>() {
                        if let Some(obj) = select.item(select.selected()) {
                            if let Ok(item) = obj.downcast::<ItemProduct>() {
                                if let Some(model) = cb_category.model() {
                                    for idex in 0..model.n_items() {
                                        if let Some(item_c) = model.item(idex) {
                                            if let Ok(obj_item) = item_c.downcast::<ItemCategory>()
                                            {
                                                if item.category_id() == obj_item.id() {
                                                    cb_category.set_selected(idex);
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }

                                btn_save.set_action_target_value(Some(&Variant::from(true)));
                                _et_barcode.set_text(&item.barcode().to_string());
                                _et_barcode.set_sensitive(false);
                                _et_name.set_text(&item.name());
                                _et_description.set_text(&item.description());
                                rev_add.set_reveal_child(true);
                            }
                        }
                    }
                }
            }
        ));

        btn_add.connect_clicked(glib::clone!(
            #[weak] rev_add,
            #[weak] btn_save,
            #[weak] _et_barcode,
            move |_| {
            //only Revealer false
            if !rev_add.is_child_revealed() {
                rev_add.set_reveal_child(true);
                btn_save.set_action_target_value(Some(&Variant::from(false)));
                  _et_barcode.set_sensitive(true);
            }
        }));

        ProductPG {
            tool_view: _tool_view,
            db_str: db_str,
            btn_save: btn_save,
            btn_refresh: _btn_refresh,
            et_barcode: _et_barcode,
            et_name: _et_name,
            et_description: _et_description,
            cb_category: cb_category,
            btn_delete: btn_delete,
            rev_add: rev_add,
            column_view: column_view,
            path_db: db.to_string(),
            parent: _win,
        }
    }

    pub fn get_tool_view(&self) -> &ToolbarView {
        &self.tool_view
    }
}

impl UpdateViewExt for ProductPG {
    fn connect_signal_view(&self) {
        let et_barcode = self.et_barcode.clone();
        let et_name = self.et_name.clone();
        let et_description = self.et_description.clone();
        let db_str = self.db_str.to_string();
        let cb_category = self.cb_category.clone();
        let myobj = self.clone();
        let win = self.parent.clone();

                let _str_db: String = self.path_db.to_string();
                let _obj_cp: ProductPG = self.clone();
                let column_view = self.column_view.clone();
                let rev_add_cp = self.rev_add.clone();
                let win_cp_del = win.clone(); //move &win

                self.btn_delete.connect_clicked(move |_| {
                    if !rev_add_cp.is_child_revealed() {
                        if let Some(select) = column_view.model().and_downcast::<SingleSelection>()
                        {
                            if let Some(obj) = select.item(select.selected()) {
                                if let Ok(item) = obj.downcast::<ItemProduct>() {
                                    let obj_product = ProductLite::del(item.barcode());
                                    let alert = DialogAlert::new(
                                        "Do you want to do this?",
                                        "If you delete data",
                                    );

                                    let _str_db_del = _obj_cp.path_db.to_string();

                                    let _obj_cp_del = _obj_cp.clone();
                                    alert.dialog.connect_response(None, move |_, str: &str| {
                                        match str {
                                            "accept" => {
                                                let ctx: &dyn DataConext<
                                                    Output = Connection,
                                                    Error = rsqlite::Error,
                                                > = &obj_product;
                                                match ctx.delete(&_str_db_del) {
                                                    Ok(_) => _obj_cp_del.refresh(),
                                                    Err(e) => println!("{}", e),
                                                }
                                            }
                                            "cancel" => {}
                                            _ => {}
                                        }
                                    });

                                    alert.dialog.present(Some(&win_cp_del));
                                }
                            }
                        }
                    }
                });
                let button = self.btn_save.clone();
                let rvealer = self.rev_add.clone();
                self.btn_save.connect_clicked(glib::clone!(
                    #[weak]
                    et_barcode,
                    #[weak]
                    et_name,
                    #[weak]
                    et_description,
                    #[weak]
                    cb_category,
                    #[weak]
                    win,
                    #[weak]
                    button,
                    #[weak]
                    rvealer,
                    move |_| {
                        if et_barcode.text().to_string().len() > 0 {
                            let alert: DialogAlert;
                            let mut status: bool = false;
                            //valid
                            if let Some(value) = button.action_target_value(){
                               if let Some(flag) = value.get::<bool>(){
                                    status = flag;
                               }
                            }

                            if !status { //false created
                               alert =
                                DialogAlert::new("Do you want to do this?", "If you create data");
                            }
                            else{
                                 alert =
                                DialogAlert::new("Do you want to do this?", "If you update data");
                            }
                           
                            let id = cb_category
                                .selected_item()
                                .unwrap()
                                .downcast::<ItemCategory>()
                                .unwrap()
                                .id();
                            
                            let obj_product = ProductLite::new(
                                et_barcode.text().to_string(),
                                et_name.text().to_string(),
                                et_description.text().to_string(),
                                id,
                            );

                            let et_barcode = et_barcode.clone();
                            let et_name = et_name.clone();
                            let et_description = et_description.clone();
                            let db_str_cp = db_str.to_string();
                            let obj_cp = myobj.clone();
                            alert.dialog.connect_response(
                                    None,
                                    move |_, str: &str| match str {
                                        "accept" => {
                                            let ctx: &dyn DataConext<
                                                Output = Connection,
                                                Error = rsqlite::Error,
                                            > = &obj_product;

                                            let resp: Result<String, String>;

                                            if status {
                                                 match ctx.update(&db_str_cp) {
                                                    Ok(rs) => resp = Ok(rs),
                                                    Err(er) => resp =Err(er)
                                                 }
                                            }
                                            else{
                                                 match ctx.insert(&db_str_cp) {
                                                    Ok(rs) =>   resp = Ok(rs),
                                                    Err(er) => resp =Err(er)
                                                 }
                                            }
                                            match resp {
                                                Ok(rs) => {    
                                                    if status{
                                                        rvealer.set_reveal_child(false); //update ending closed
                                                    }
                                                et_barcode.set_text("");
                                                et_name.set_text("");
                                                et_description.set_text("");
                                                et_barcode.set_sensitive(true);
                                                obj_cp.refresh();
                                                println!("DB messenger : {}", rs);
                                                },
                                                Err(rs) => {
                                                    println!("DB messenger Fail: {}", rs);
                                                }
                                            }
                                        }
                                        "cancel" => {}
                                        _ => {}
                                    },
                                );

                            alert.dialog.present(Some(&win));
                        }
                    }
                ));
            
        let my_obj = self.clone();
        self.btn_refresh.connect_clicked(move |_| {
            my_obj.refresh();
        });
    }

    fn refresh(&self) {
        let store = ListStore::builder()
            .item_type(ItemProduct::static_type())
            .build();

        let obj = ProductLite::empty();
        let query: &dyn QueryContext<Output = Option<Vec<ProductLite>>> = &obj;

        // loaded category
        let filter_cat: &dyn QueryFilterContext<Output = Option<Vec<CategoryLite>>> =
            &CategoryLite::empty();

        let store_cbox = ListStore::builder()
            .item_type(ItemCategory::static_type())
            .build();

        match filter_cat.get_filter(self.db_str.as_str()){
            Some(list) => {
                  for item in list.iter() {
                store_cbox.append(&ItemCategory::new(
                    item.id,
                    item.name.to_string(),
                    "".to_string(),
                    None::<DateTime>,
                    None::<DateTime>,
                ));
            }
            },
            None => {
                 println!("pasa");
            },
        }

        let select_cb = SingleSelection::new(Some(store_cbox));
        self.cb_category.set_model(Some(&select_cb));

        let factory_name_cb = SignalListItemFactory::new();
        factory_name_cb.connect_setup(|_, obj| {
            let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
            let cbox = Box::new(gtk::Orientation::Horizontal, 10);
            let label_id = Label::new(Some(""));
            let label_name = Label::new(Some(""));
            cbox.append(&label_id);
            cbox.append(&label_name);
            list_item.set_child(Some(&cbox));
        });

        factory_name_cb.connect_bind(|_, obj| {
            let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
            let cbox = list_item.child().unwrap().downcast::<Box>().unwrap();
            let label_id = cbox
                .observe_children()
                .item(0)
                .unwrap()
                .downcast::<Label>()
                .unwrap();

            let label_name = cbox
                .observe_children()
                .item(1)
                .unwrap()
                .downcast::<Label>()
                .unwrap();

            let item = list_item.item().and_downcast::<ItemCategory>().unwrap();
            label_id.set_text(&item.id().to_string());
            label_name.set_text(&item.name());
        });

        self.cb_category.set_factory(Some(&factory_name_cb));
        // end loaded category

        if let Some(list) = query.get_all(self.path_db.as_str()) {
            for item in list.iter() {
                let date_created: Option<DateTime>;
                let date_updated: Option<DateTime>;

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

                let product = ItemProduct::new(
                    item.barcode.to_string(),
                    item.name.to_string(),
                    item.description.to_string(),
                    item.categoryid,
                    date_created,
                    date_updated,
                );

                product.set_is_created(true);

                store.append(&product);
            }
        }

        let select = SingleSelection::new(Some(store));
        self.column_view.set_model(Some(&select));

        let factory_barcode = SignalListItemFactory::new();
        factory_barcode.connect_setup(|_, obj| {
            let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
            let label = Label::new(Some(""));
            list_item.set_child(Some(&label));
        });
        factory_barcode.connect_bind(|_, obj| {
            let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
            let label = list_item.child().unwrap().downcast::<gtk::Label>().unwrap();
            let item = list_item.item().and_downcast::<ItemProduct>().unwrap();
            label.set_text(&item.barcode());
        });

        let factory_name = SignalListItemFactory::new();

        factory_name.connect_setup(|_, obj| {
            let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
            let label = Label::new(Some(""));
            list_item.set_child(Some(&label));
        });
        factory_name.connect_bind(|_, obj| {
            let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
            let label = list_item.child().unwrap().downcast::<gtk::Label>().unwrap();
            let item = list_item.item().and_downcast::<ItemProduct>().unwrap();
            label.set_text(&item.name());
        });

        let factory_description = SignalListItemFactory::new();
        factory_description.connect_setup(|_, obj| {
            let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
            let label = Label::new(Some(""));
            list_item.set_child(Some(&label));
        });
        factory_description.connect_bind(|_, obj| {
            let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
            let label = list_item.child().unwrap().downcast::<gtk::Label>().unwrap();
            let item = list_item.item().and_downcast::<ItemProduct>().unwrap();
            label.set_text(&item.description());
        });

        let factory_category = SignalListItemFactory::new();

        factory_category.connect_setup(|_, obj| {
            let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
            let label = Label::new(Some(""));
            list_item.set_child(Some(&label));
        });

        factory_category.connect_bind(|_, obj| {
            let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
            let label = list_item.child().unwrap().downcast::<gtk::Label>().unwrap();
            let item = list_item.item().and_downcast::<ItemProduct>().unwrap();
            label.set_text(&item.category_id().to_string());
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
            let item = list_item.item().and_downcast::<ItemProduct>().unwrap();
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
            let item = list_item.item().and_downcast::<ItemProduct>().unwrap();
            if let Some(date_updated) = item.date_updated() {
                label.set_text(&date_updated.format("%Y-%m-%d %H:%M:%S").unwrap());
            }
        });

        let col_id = ColumnViewColumn::new(Some("Id"), Some(factory_barcode));
        let col_name = ColumnViewColumn::new(Some("Name"), Some(factory_name));
        let col_description = ColumnViewColumn::new(Some("Description"), Some(factory_description));
        let col_category_id = ColumnViewColumn::new(Some("Category"), Some(factory_category));
        let col_date_created = ColumnViewColumn::new(Some("Date Created"), Some(factory_created));
        let col_date_updated = ColumnViewColumn::new(Some("Date Updated"), Some(factory_updated));

        col_id.set_fixed_width(90);
        col_id.set_resizable(true);
        col_id.set_expand(true);

        col_name.set_fixed_width(120);
        col_name.set_resizable(true);
        col_name.set_expand(true);

        col_description.set_fixed_width(200);
        col_description.set_resizable(true);
        col_description.set_expand(true);
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
        self.column_view.append_column(&col_name);
        self.column_view.append_column(&col_description);
        //let view = SortListModel::new(model, sorter)
        self.column_view.append_column(&col_category_id);
        self.column_view.append_column(&col_date_created);
        self.column_view.append_column(&col_date_updated);
    }
}
