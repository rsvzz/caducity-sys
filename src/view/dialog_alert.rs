use adw::{AlertDialog, prelude::*};
pub struct DialogAlert {
    pub dialog: AlertDialog, 
}

impl DialogAlert {
    pub fn new(title: &str, body: &str) -> Self {
        let _alert = AlertDialog::new(Some(title), Some(body));
        _alert.add_response("cancel", "Cancel");
        _alert.add_response("accept", "Accept");

        _alert.set_response_appearance("accept", adw::ResponseAppearance::Destructive);
        
        DialogAlert { dialog: _alert,}
    }

    pub fn set_body(&self, body: &str){
        self.dialog.set_body(body);
    }

    pub fn set_remove_response(&self, name: &str){
        self.dialog.remove_response(name);
    }
}
