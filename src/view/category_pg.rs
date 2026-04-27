use crate::data::CategoryLite;
use crate::model::{DataConext, ItemCategory, LoadUI, QueryContext, CreateLiteExt};
use crate::view::{DialogAlert, UpdateViewExt};
use adw::{ApplicationWindow, EntryRow, ToolbarView, prelude::*};
use chrono::Local;
use glib::DateTime;
use gtk::gio::ListStore;
use gtk::glib;
use gtk::{
    Builder, Button, ColumnView, ColumnViewColumn, Label, Revealer, SignalListItemFactory,
    SingleSelection,
};
use rsqlite::Connection;

#[derive(Clone)]
pub struct CategoryPG {
    tool_view: ToolbarView,
    column_view: ColumnView,
    path_db: String,
    pub btn_refresh: Button,
    btn_save: Button,
    btn_delete: Button,
    et_id: EntryRow,
    et_name: EntryRow,
    et_description: EntryRow,
    rev_add: Revealer,
    parent: ApplicationWindow,
}

impl CategoryPG {
    pub fn new(ui: &LoadUI, db: &str, win: &ApplicationWindow) -> Self {

        let _build: Builder = ui.build.clone();
        let _window = win.clone();
        let _tool_view: ToolbarView = _build.object("tool_view").unwrap();
        let btn_add: Button = _build.object("btn_show_add").unwrap();
        let btn_refresh: Button = _build.object("btn_refresh").unwrap();
        let btn_edit: Button = _build.object("btn_edit").unwrap();
        let column_view: ColumnView = _build.object("view_list").unwrap();
        let rev_add: Revealer = _build.object("rev_add").unwrap();

        let btn_cancel: Button = _build.object("btn_cancel").unwrap();
        let btn_delete: Button = _build.object("btn_delete").unwrap();
        let btn_save: Button = _build.object("btn_save").unwrap();

        let et_cateid: EntryRow = _build.object("et_cateid").unwrap();
        let et_name: EntryRow = _build.object("et_name").unwrap();
        let et_description: EntryRow = _build.object("et_description").unwrap();

        btn_edit.connect_clicked(glib::clone!(
            #[weak]
            column_view,
            #[weak]
            rev_add,
            #[weak]
            et_cateid,
            #[weak]
            et_name,
            #[weak]
            et_description,
            move |_| {
                if !rev_add.is_child_revealed() {
                    if let Some(select) = column_view.model().and_downcast::<SingleSelection>() {
                        if let Some(obj) = select.item(select.selected()) {
                            if let Ok(item) = obj.downcast::<ItemCategory>() {
                                //println!("{} - {}", item.id(), item.name());
                                et_cateid.set_text(&item.id().to_string());
                                et_name.set_text(&item.name());
                                et_description.set_text(&item.description());
                                rev_add.set_reveal_child(true);
                            }
                        }
                    }
                }
            }
        ));
        btn_cancel.connect_clicked(glib::clone!(
            #[weak]
            rev_add,
            #[weak]
            et_cateid,
            #[weak]
            et_name,
            #[weak]
            et_description,
            move |_| {
                rev_add.set_reveal_child(false);
                et_cateid.set_text("");
                et_name.set_text("");
                et_description.set_text("");
            }
        ));

        btn_add.connect_clicked(glib::clone!(
            #[weak]
            rev_add,
            move |_| {
                if !rev_add.is_child_revealed() {
                    rev_add.set_reveal_child(!rev_add.reveals_child());
                }
            }
        ));

        CategoryPG {
            tool_view: _tool_view,
            column_view: column_view,
            path_db: db.to_string(),
            btn_refresh: btn_refresh,
            btn_save: btn_save,
            et_id: et_cateid,
            et_name: et_name,
            et_description: et_description,
            rev_add: rev_add,
            btn_delete: btn_delete,
            parent: _window,
        }
    }

    pub fn get_tool_view(&self) -> &ToolbarView {
        &self.tool_view
    }
}

impl UpdateViewExt for CategoryPG {
    fn connect_signal_view(&self) {
              let db_str = self.path_db.to_string();
        let et_id = self.et_id.clone();
        let et_name = self.et_name.clone();
        let et_description = self.et_description.clone();
        let rev_up = self.rev_add.clone();
        let obj = self.clone();
        let win = self.parent.clone();
                self.btn_save.connect_clicked(glib::clone!(
                    #[weak]
                    et_id,
                    #[weak]
                    et_name,
                    #[weak]
                    et_description,
                    #[weak]
                    rev_up,
                    #[weak]
                    win,
                    move |_| {
                        //let status = Rc::new(RefCell::new(false));
                        if et_id.text().to_string().len() == 0 {
                            let obj_cate = CategoryLite::new(
                                et_name.text().to_string(),
                                et_description.text().to_string(),
                            );

                            let alert =
                                DialogAlert::new("Do you want to do this?", "If you create data");

                            let db_str_cp = db_str.to_string();
                            let obj_cp = obj.clone();
                            let et_name_cp = et_name.clone();
                            let et_description_cp = et_description.clone();
                            alert
                                .dialog
                                .connect_response(None, move |_, str: &str| match str {
                                    "accept" => {
                                        let ctx: &dyn DataConext<Output = Connection, Error = rsqlite::Error> = &obj_cate;
                                        if let Ok(_) = ctx.insert(&db_str_cp){
                                              et_name_cp.set_text("");
                                        et_description_cp.set_text("");
                                        obj_cp.refresh();
                                        }
                                      
                                    }
                                    "cancel" => {}
                                    _ => {}
                                });

                            alert.dialog.present(Some(&win));
                        } else {
                            if let Ok(id) = et_id.text().parse::<u32>() {
                                let obj_cate = CategoryLite::data_new(
                                    id,
                                    et_name.text().to_string(),
                                    et_description.text().to_string(),
                                    Local::now().naive_local(),
                                    Local::now().naive_local(),
                                );

                                let alert = DialogAlert::new(
                                    "Do you want to do this?",
                                    "If you update data",
                                );

                                let db_str_cp = db_str.to_string();
                                let obj_cp = obj.clone();
                                let et_id_cp = et_id.clone();
                                let et_name_cp = et_name.clone();
                                let et_description_cp = et_description.clone();
                                alert.dialog.connect_response(
                                    None,
                                    move |_, str: &str| match str {
                                        "accept" => {
                                            let ctx: &dyn DataConext<Output = Connection, Error = rsqlite::Error> = &obj_cate;
                                            if let Ok(_)  = ctx.update(&db_str_cp){
                                                et_id_cp.set_text("");
                                                et_name_cp.set_text("");
                                                et_description_cp.set_text("");
                                                obj_cp.refresh();
                                                rev_up.set_reveal_child(false);
                                            }
                                           
                                        }
                                        "cancel" => {}
                                        _ => {}
                                    },
                                );

                                alert.dialog.present(Some(&win));
                            }
                        }
                    }
                ));

        let _str_db = self.path_db.to_string();
        let _obj_cp: CategoryPG = self.clone();
        let column_view = self.column_view.clone();
        let rev_add_cp = self.rev_add.clone();
        let win_cp = win.clone();
        self.btn_delete.connect_clicked(move |_| {
            if !rev_add_cp.is_child_revealed() {
                if let Some(select) = column_view.model().and_downcast::<SingleSelection>() {
                    if let Some(obj) = select.item(select.selected()) {
                        if let Ok(item) = obj.downcast::<ItemCategory>() {
                            let obj_cate = CategoryLite::del(item.id());
                            let alert =
                                DialogAlert::new("Do you want to do this?", "If you delete data");

        
                let _str_db_del = _obj_cp.path_db.to_string();

                let _obj_cp_del = _obj_cp.clone();
                  alert.dialog.connect_response(
                                    None,
                                    move |_, str: &str| match str {
                                        "accept" => {
                                            let ctx: &dyn DataConext<Output = Connection, Error = rsqlite::Error> = &obj_cate;
                                             match ctx.delete(&_str_db_del){
                                                Ok(_) => _obj_cp_del.refresh(),
                                                Err(e) => println!("{}", e)
                                             }
                                                
                                        }
                                        "cancel" => {}
                                        _ => {}
                                    },
                                );

                                 alert.dialog.present(Some(&win_cp));
            }
        }
                          

                               
                            }
                        }
              
        });
    }

    fn refresh(&self) {
         let store = ListStore::builder()
            .item_type(ItemCategory::static_type())
            .build();
        let obj = CategoryLite::empty();
        let query: &dyn QueryContext<Output = Option<Vec<CategoryLite>>> = &obj;

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

                store.append(&ItemCategory::new(
                    item.id,
                    item.name.to_string(),
                    item.description.to_string(),
                    date_created,
                    date_updated,
                ));
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
            let item = list_item.item().and_downcast::<ItemCategory>().unwrap();
            label.set_text(&item.id().to_string());
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
            let item = list_item.item().and_downcast::<ItemCategory>().unwrap();
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
            let item = list_item.item().and_downcast::<ItemCategory>().unwrap();
            label.set_text(&item.description());
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
            let item = list_item.item().and_downcast::<ItemCategory>().unwrap();
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
            let item = list_item.item().and_downcast::<ItemCategory>().unwrap();
            if let Some(date_updated) = item.date_updated() {
                label.set_text(&date_updated.format("%Y-%m-%d %H:%M:%S").unwrap());
            }
        });

        let col_id = ColumnViewColumn::new(Some("Id"), Some(factory_id));
        let col_name = ColumnViewColumn::new(Some("Name"), Some(factory_name));
        let col_description = ColumnViewColumn::new(Some("Description"), Some(factory_description));
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
        self.column_view.append_column(&col_date_created);
        self.column_view.append_column(&col_date_updated);
    }
}