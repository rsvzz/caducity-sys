use adw::{Application, ApplicationWindow, NavigationPage, prelude::*};
use gtk::gio::ListStore;
use gtk::{
    Align, Box, Builder, Image, Label, ListView, Orientation, SignalListItemFactory,
    SingleSelection,
};
use std::env;

mod data;
mod model;
mod view;

use model::{ItemList, LoadUI};
use view::{CategoryPG, HomePG, ProductPG, UpdateViewExt};

fn main() {
    //let _ = gtk::init(); //need CssProvider
    let app: Application = Application::builder()
        .application_id("io.github.name.newapp")
        .build();

    let path = env::current_exe().expect("No path exe");

    app.connect_activate({
        let dir = path.clone();
        move |app| {
            let main_ui: &str;
            let category_ui: &str;
            let product_ui: &str;
            let inventory_ui: &str;
            let home_ui: &str;

            if cfg!(debug_assertions) {
                home_ui = "../../data/ui/home.ui";
                main_ui = "../../data/ui/main.ui"; //devmode
                category_ui = "../../data/ui/category.ui"; //devmode
                product_ui = "../../data/ui/product.ui"; //devmode
                inventory_ui = "../../data/ui/inventory.ui"; //devmode
            } else {
                home_ui = "../share/caducity-sys/ui/home.ui"; //release
                main_ui = "../share/caducity-sys/ui/main.ui"; //release
                category_ui = "../share/caducity-sys/ui/category.ui"; //release
                product_ui = "../share/caducity-sys/ui/product.ui"; //release
                inventory_ui = "../share/caducity-sys/ui/inventory.ui"; //release
            }

            let main = LoadUI::get_ui(main_ui, &dir);
            let cate = LoadUI::get_ui(category_ui, &dir);
            let product = LoadUI::get_ui(product_ui, &dir);
            let inventory = LoadUI::get_ui(inventory_ui, &dir);
            let home = LoadUI::get_ui(home_ui, &dir);

            let build: Builder = main.build;

            let window: ApplicationWindow = build.object("adw_app").unwrap();
            let list_view: ListView = build.object("list_option_view").unwrap();
            let nav_page_content: NavigationPage = build.object("nav_page_content").unwrap();

            //path db
            let data_home = env::var("XDG_DATA_HOME")
                .unwrap_or_else(|_| format!("{}/.local/share", env::var("HOME").unwrap()));

            let app_dir = format!("{}/caducity-sys", data_home);
            std::fs::create_dir_all(&app_dir).expect("No se pudo crear el directorio");

            let db_path = format!("{}/caducity.sqlite3", app_dir);
            println!("{}", db_path);
            let pg_category = CategoryPG::new(&cate, &db_path, &window);
            let pg_product = ProductPG::new(&product, &db_path, &window);
            let pg_home = HomePG::new(&home, &db_path, &inventory, &window);

            nav_page_content.set_child(Some(pg_home.get_tool_view()));
            //load signal pages
            pg_category.connect_signal_view();
            pg_product.connect_signal_view();
            pg_home.connect_signal_view();
            pg_home.refresh(); //first page refresh

            window.set_application(Some(app));

            let store = ListStore::builder()
                .item_type(ItemList::static_type())
                .build();

            let factory = SignalListItemFactory::new();

            factory.connect_setup(move |_, obj| {
                let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
                let label = Label::new(Some(""));
                let box_content: Box = Box::builder()
                    .orientation(Orientation::Horizontal)
                    .spacing(5)
                    .margin_top(5)
                    .margin_bottom(5)
                    .build();

                let icon = Image::from_icon_name("go-next-symbolic");
                label.set_halign(Align::Start);
                label.set_margin_start(10);
                icon.set_halign(Align::End);
                icon.set_hexpand(true);
                box_content.append(&label);
                box_content.append(&icon);
                list_item.set_child(Some(&box_content));
            });

            factory.connect_bind(move |_, obj| {
                let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
                let box_content: Box = list_item.child().and_downcast::<Box>().unwrap();
                let item = list_item.item().and_downcast::<ItemList>().unwrap();
                if let Some(child) = box_content.first_child() {
                    if let Ok(label) = child.downcast::<Label>() {
                        label.set_text(&item.name());
                    }
                }
            });

            //add items
            store.append(&ItemList::new("Home".to_string()));
            store.append(&ItemList::new("Categories".to_string()));
            store.append(&ItemList::new("Products".to_string()));

            let selectmode: SingleSelection = SingleSelection::new(Some(store));

            let cate_page = pg_category.clone();
            let prod_page = pg_product.clone();
            //let inv_page = pg_inventory.clone();

            let page_content = nav_page_content.clone();
            let pg_home_cp = pg_home.clone();
            selectmode.connect_selected_notify(move |sel| {
                let pos = sel.selected();
                if pos != gtk::INVALID_LIST_POSITION {
                    let item = sel.selected_item().and_downcast::<ItemList>().unwrap();

                    match item.name().as_str() {
                        "Home" => {
                            page_content.set_child(Some(pg_home_cp.get_tool_view()));
                            let up_view: &dyn UpdateViewExt = &pg_home_cp;
                            up_view.refresh();
                        }
                        "Categories" => {
                            page_content.set_child(Some(cate_page.get_tool_view()));
                            cate_page.refresh();

                            let page = cate_page.clone();
                            cate_page.btn_refresh.connect_clicked(move |_| {
                                page.refresh();
                            });
                        }
                        "Products" => {
                            page_content.set_child(Some(prod_page.get_tool_view()));
                            let up_view: &dyn UpdateViewExt = &prod_page;
                            //up_view.connect_signal_view();
                            up_view.refresh();
                        }
                        _ => println!("{}", "Nothing"),
                    }
                }
            });

            list_view.set_factory(Some(&factory));
            list_view.set_model(Some(&selectmode));

            window.present();
        }
    });

    app.run();
}
