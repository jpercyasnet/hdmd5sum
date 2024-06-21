use std::path::Path;

pub fn execpress (hddir: String, targetdir: String, refname: String, targetname: String) -> (u32, String) {
     let mut errcode: u32 = 0;
     let mut errstring: String = "all good and now process execution".to_string();
     if Path::new(&targetdir).exists() {
         if Path::new(&hddir).exists() {
             if refname.len() < 4 {
                 errstring = "the reference name is less than 4 characters".to_string();
                 errcode = 1;
             } else {
                 if !targetname.contains(".") { 
                     errstring = "target name does not have a file type (ie xx.xxx)".to_string();
                     errcode = 2;
                 } else {
                     let lrperpos = targetname.rfind(".").unwrap();
                     if (targetname.len() - lrperpos) < 4 {
                         errstring = "target name does not have a valid type (must be at least 3 characters".to_string();
                         errcode = 3;
                     } else {
                         let lfperpos = targetname.find(".").unwrap();
                         if lfperpos < 3 {
                             errstring = "target name is least than 3 characters".to_string();
                             errcode = 4;
                         } else {
                             let targetfullname: String = format!("{}/{}", targetdir, targetname);
                             if Path::new(&targetfullname).exists() {
                                 errstring = "the target output file already exists".to_string();
                                 errcode = 5;
                             }
                         }
                     }
                 }
             }
         } else {
             errstring = "the harddrive directory does not exist".to_string();
             errcode = 6;
         }
     } else {
         errstring = "the target directory does not exist".to_string();
         errcode = 7;
     }
     (errcode, errstring)
}
