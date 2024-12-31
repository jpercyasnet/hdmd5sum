use std::process::Command as stdCommand;
use std::process::Stdio;

pub fn get_winsize () -> (u32, String, u32, u32) {
    let mut errcode: u32 = 0;
    let mut errstring: String = "no error".to_string();
    let mut widtho: u32 = 0;
    let mut heighto: u32 = 0;

    let output1 = stdCommand::new("sh")
                     .arg("-c")
                     .arg("winitsize01")
                     .stdout(Stdio::piped())
                     .output()
                     .expect("failed to execute process");
    let mut output = String::from_utf8(output1.stdout).unwrap();
    let lenoutput = output.len();
    if lenoutput < 25 {
        let locind = output.find("Current mode: ");
        if locind != None {
           let start = locind.unwrap() + 14;
           let mut end = start + 11;
           if end > lenoutput {
               end = lenoutput;
           }
           let getseg1 = output.get(start..end).expect("REASON").to_string();
           let locx = getseg1.find("x");
           if locx != None {
               let end1 = locx.unwrap();
               let start2 = end1 + 1;
               let end2 = getseg1.len() - 1;
               let widths = getseg1.get(0..end1);
               let heights = getseg1.get(start2..end2);
               println!("width height: -{:?}- -{:?}-", widths, heights);
               let widths_int: i32 = widths.unwrap().parse().unwrap_or(-99);
               let heights_int: i32 = heights.unwrap().parse().unwrap_or(-99);
               if widths_int > 20 {
                   if heights_int > 75 {
                       widtho = widths_int as u32;
                       heighto = heights_int as u32;
                       errstring = format!("screen size {} x {}", widtho, heighto);
                       errcode = 0;
                   } else {
                       errstring = format!("Invalid segment- height of {}",heights_int);
                       errcode = 8;
                   }
               } else {
                   errstring = format!("Invalid segment- width of {}", widths_int);
                   errcode = 9;
               }
           } else {
               errstring = format!("Invalid segment- no x {}", getseg1);
               errcode = 1;
           }
       } else {
           errstring = format!("Invalid segment- no current {}", output);
           errcode = 2;
       }
    } else {
        let mut contloop = true;
        let mut foundsize = false;
        while contloop {
           let locind = output.find("Current mode: ");
           if locind != None {
               let start = locind.unwrap() + 15;
               let end = start + 11;
               let getseg1 = output.get(start..end).expect("REASON").to_string();
               let locx = getseg1.find("x");
               if locx != None {
                   let end1 = locx.unwrap();
                   let start2 = end1 + 1;
                   let locy = getseg1.find("+");
                   if locy != None {
                       let end2 = locy.unwrap();
                       let widths = getseg1.get(0..end1);
                       let heights = getseg1.get(start2..end2);
                       let widths_int: i32 = widths.unwrap().parse().unwrap_or(-99);
                       let heights_int: i32 = heights.unwrap().parse().unwrap_or(-99);
                       if widths_int > 20 {
                           if heights_int > 75 {
                               if foundsize {
                                   if widtho > widths_int as u32 {
                                       widtho = widths_int as u32;
                                   }  
                                   if heighto > heights_int as u32 {
                                       heighto = heights_int as u32;
                                   } 
                               } else { 
                                   widtho = widths_int as u32;
                                   heighto = heights_int as u32;
                                   foundsize = true;
                               }
                               errstring = format!("screen size {} x {}", widtho, heighto);
                               errcode = 0;
                               output = output.get(end..).expect("REASON").to_string();
                           } else {
                               errstring = format!("Invalid height of {:?} -- {}", heights, getseg1);
                               errcode = 3;
                               contloop = false;
                           }
                       } else {
                           errstring = format!("Invalid width size {:?} x {:?} -- {}", widths, heights, getseg1);
                           errcode = 4;
                           contloop = false;
                       }
                   } else {
                       errstring = format!("Invalid segment- no + {}", getseg1);
                       errcode = 5;
                       contloop = false;
                   }
               } else {
                   errstring = format!("Invalid segment- no x {}", getseg1);
                   errcode = 6;
                   contloop = false;
               }
           } else {
               if !foundsize {
                   errstring = format!("Invalid output - no current {:?}", output.get(1..30));
                   errcode = 7;
               }
               contloop = false;
           }
        }
    }
    (errcode, errstring, widtho, heighto)
}

