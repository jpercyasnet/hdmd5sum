use rfd::FileDialog;
use std::path::{Path};
pub fn inputpress (inputval: String) -> (u32, String, String) {
     let errcode: u32;
     let errstring: String;
     let mut new_input: String;
     if Path::new(&inputval).exists() {
         new_input = inputval.to_string();
     } else {
         new_input = "/".to_string();
     }
     let newfile = FileDialog::new()
//        .set_location(&new_input)
//        .show_open_single_dir()
//        .unwrap();
         .set_directory(&new_input)
         .pick_folder();
     if newfile == None {
         errstring = "error getting directory -- possible cancel key hit".to_string();
         errcode = 1;
     } else {
         new_input = newfile.as_ref().expect("REASON").display().to_string();
         errstring = "got directory".to_string();
         errcode = 0;
     } 
    (errcode, errstring, new_input)
}

