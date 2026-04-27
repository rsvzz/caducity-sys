use crate::data::{CategoryLite, InventoryDetLite, InventoryLite, ProductLite};
use crate::model::{
    CreateLiteExt, DataConext, ItemHome, LoadUI, QueryContext, QueryValidExtContext,
};
use crate::view::{DialogAlert, InventoryPG, UpdateViewExt};
use adw::prelude::{AdwDialogExt, NavigationPageExt};
use adw::{ApplicationWindow, NavigationPage, NavigationView};
use gtk::gio::ListStore;
use gtk::glib::DateTime;
use gtk::{
    Builder, Button, ColumnView, ColumnViewColumn, Label, SignalListItemFactory, SingleSelection,
    prelude::*,
};
use gtk::{Image, glib};
use rsqlite::Connection;

#[derive(Clone)]
pub struct HomePG {
    tool_view: NavigationView,
    path_db: String,
    column_view: ColumnView,
    ui_detail: LoadUI,
    page_detail: NavigationPage,
    btn_show_detail: Button,
    btn_add: Button,
    btn_closed: Button,
    parent: ApplicationWindow,
}

impl HomePG {
    pub fn new(ui: &LoadUI, db: &str, inv_ui: &LoadUI, win: &ApplicationWindow) -> Self {
        let window = win.clone();
        
        let ctx_cate: &dyn DataConext<Output = Connection, Error = rsqlite::Error> =
            &CategoryLite::empty();
        let ctx_prod: &dyn DataConext<Output = Connection, Error = rsqlite::Error> =
            &ProductLite::empty();
        let ctx_inv: &dyn DataConext<Output = Connection, Error = rsqlite::Error> =
            &InventoryLite::empty();
        let ctx_inv_det: &dyn DataConext<Output = Connection, Error = rsqlite::Error> =
            &InventoryDetLite::empty();

        let _ = ctx_cate.create(db);
        let _ = ctx_prod.create(db);
        let _ = ctx_inv.create(db);
        let _ = ctx_inv_det.create(db);

        let _build: Builder = ui.build.clone();
        let _tool_view: NavigationView = _build.object("nav_view").unwrap();
        let btn_add: Button = _build.object("btn_show_add").unwrap();
        let btn_add_list: Button = _build.object("btn_add_list").unwrap();
        let btn_closed: Button = _build.object("btn_closed").unwrap();

        let column_view: ColumnView = _build.object("view_list").unwrap();
        //let model_status_inv: StringList = _build.object("model_status_inv").unwrap();
        let page: NavigationPage = _build.object("page_content").unwrap();

        HomePG {
            tool_view: _tool_view,
            path_db: db.to_string(),
            column_view: column_view,
            ui_detail: inv_ui.clone(),
            page_detail: page,
            btn_show_detail: btn_add_list,
            btn_add: btn_add,
            btn_closed: btn_closed,
            parent: window,
        }
    }

    pub fn get_tool_view(&self) -> &NavigationView {
        &self.tool_view
    }
}

impl UpdateViewExt for HomePG {
    fn connect_signal_view(&self) {
        let page_cp: NavigationPage = self.page_detail.clone();
        let nav_view_cp = self.get_tool_view().clone();
        let col_view = self.column_view.clone();
        let ui_detail = self.ui_detail.clone();
        let db_path_str = self.path_db.to_string();
        let win = self.parent.clone();
        self.btn_show_detail.connect_clicked(glib::clone!(
            #[weak]
            page_cp,
            #[weak]
            nav_view_cp,
            #[weak]
            col_view,
            #[weak]
            win,
            move |_| {
                if let Some(select_cp) = col_view.model().and_downcast::<SingleSelection>() {
                    if let Some(obj) = select_cp.selected_item() {
                        if let Ok(item) = obj.downcast::<ItemHome>() {
                            let alert = DialogAlert::new("What do you do?", "");
                            alert.set_remove_response("cancel");
                            match item.statusid() {
                                0 => alert.set_body("status closed dont change this"),
                                1 => {
                                    if let Some(root) = nav_view_cp.root() {
                                        if let Some(parent) =
                                            root.downcast_ref::<ApplicationWindow>()
                                        {
                                            let _parent = parent.clone();
                                            let pg_inventory = InventoryPG::new(
                                                &ui_detail,
                                                &db_path_str,
                                                item.id(),
                                                &_parent,
                                            );
                                            let up_view: &dyn UpdateViewExt = &pg_inventory;
                                            up_view.connect_signal_view();
                                            up_view.refresh();
                                            page_cp.set_child(Some(pg_inventory.get_tool_view()));
                                            nav_view_cp.push(&page_cp);
                                        }
                                    }
                                }
                                _ => alert.set_body("this option dont found"),
                            }

                            alert.dialog.present(Some(&win));
                        }
                    }
                }
            }
        ));

        let db_path_str_cp = self.path_db.to_string();
        let obj_cp_add = self.clone();
        self.btn_add.connect_clicked(move |_| {
            let query: &dyn QueryValidExtContext<Output = Option<u8>> = &InventoryLite::empty();
            if let Some(data) = query.get_filter(&db_path_str_cp) {
                println!("open inventory first closed {}", data.to_string());
            } else {
                let obj = InventoryLite::new();
                let _ = obj.insert(&db_path_str_cp);
                obj_cp_add.refresh();
            }
        });

        let db_str_cp = self.path_db.to_string();
        let obj_cp = self.clone();

        self.btn_closed.connect_clicked(glib::clone!(
            #[weak]
            col_view,
            #[weak]
            win,
            move |_| {
                if let Some(select_cp) = col_view.model().and_downcast::<SingleSelection>() {
                    if let Some(obj) = select_cp.selected_item() {
                        if let Ok(item) = obj.downcast::<ItemHome>() {
                            let alert = DialogAlert::new("What do you do?", "What do you do?");
                            alert.set_remove_response("cancel");
                            match item.statusid() {
                                0 => {
                                    alert.set_body(
                                        format!("status {} is closed", item.statusid().to_string())
                                            .as_str(),
                                    );
                                }
                                1 => {
                                    let inventory = InventoryLite::upd(item.id());
                                    //let ctx: &dyn DataConext<Output = Connection, Error= rsqlite::Error> = &inventory;
                                    match inventory.update(&db_str_cp) {
                                        Ok(obj) => {
                                            alert.set_body(format!("update {}", obj).as_str());
                                            obj_cp.refresh();
                                        }
                                        Err(_) => {
                                            alert.set_body("update fail");
                                        }
                                    }
                                }
                                _ => {
                                    alert.set_body(
                                        format!("status {} no exist info.", item.statusid())
                                            .as_str(),
                                    );
                                }
                            }

                            alert.dialog.present(Some(&win));
                        }
                    }
                }
            }
        ));
    }

    fn refresh(&self) {
        let store = ListStore::builder()
            .item_type(ItemHome::static_type())
            .build();

        let query: &dyn QueryContext<Output = Option<Vec<InventoryLite>>> = &InventoryLite::empty();

        match query.get_all(self.path_db.as_str()) {
            Some(list) => {
                for item in list.iter() {
                    let date_created: Option<DateTime>;
                    let date_updated: Option<DateTime>;

                    if let Some(date_upd) = item.date_updated {
                        date_updated = Some(
                            DateTime::from_unix_utc(date_upd.and_utc().timestamp())
                                .expect("REASON"),
                        );
                    } else {
                        date_updated = None::<DateTime>;
                    }

                    if let Some(date_cre) = item.date_created {
                        date_created = Some(
                            DateTime::from_unix_utc(date_cre.and_utc().timestamp())
                                .expect("REASON"),
                        );
                    } else {
                        date_created = None::<DateTime>;
                    }

                    let home = ItemHome::new(item.id, item.statusid, date_created, date_updated);
                    store.append(&home);
                }
            }
            None => println!("Inventory None"),
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
            let item = list_item.item().and_downcast::<ItemHome>().unwrap();
            label.set_text(&item.id().to_string());
        });

        let factory_status = SignalListItemFactory::new();
        factory_status.connect_setup(|_, obj| {
            let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
            let image = Image::new();
            image.set_pixel_size(24);
            //let label = Label::new(Some(""));
            list_item.set_child(Some(&image));
        });

        factory_status.connect_bind(move |_, obj| {
            let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
            let image = list_item.child().unwrap().downcast::<gtk::Image>().unwrap();
            let item = list_item.item().and_downcast::<ItemHome>().unwrap();

            //if let Some(name) = status_inv.string(item.statusid() as u32) {
            match item.statusid() {
                0 => {
                    image.set_icon_name(Some("changes-prevent-symbolic"));
                    image.set_tooltip_text(Some("Open"));
                }
                1 => {
                    image.set_icon_name(Some("changes-allow-symbolic"));
                    image.set_tooltip_text(Some("Closed"));
                }
                _ => image.set_icon_name(Some("changes-allow-symbolic")), //no added
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
            let item = list_item.item().and_downcast::<ItemHome>().unwrap();
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
            let item = list_item.item().and_downcast::<ItemHome>().unwrap();
            if let Some(date_updated) = item.date_updated() {
                label.set_text(&date_updated.format("%Y-%m-%d %H:%M:%S").unwrap());
            }
        });

        let col_id = ColumnViewColumn::new(Some("Id"), Some(factory_id));
        let col_status = ColumnViewColumn::new(Some("Status"), Some(factory_status));
        let col_date_created = ColumnViewColumn::new(Some("Date Created"), Some(factory_created));
        let col_date_updated = ColumnViewColumn::new(Some("Date Updated"), Some(factory_updated));

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
        self.column_view.append_column(&col_status);
        self.column_view.append_column(&col_date_created);
        self.column_view.append_column(&col_date_updated);
    }
}
