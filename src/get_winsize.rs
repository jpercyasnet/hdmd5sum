use std::process::Command as stdCommand;
use std::process::Stdio;

pub fn get_winsize () -> (u32, String, u32, u32) {
    let errcode: u32;
    let errstring: String;
    let mut widtho: u32 = 0;
    let mut heighto: u32 = 0;

    let output1 = stdCommand::new("sh")
                     .arg("-c")
                     .arg("xrandr")
                     .stdout(Stdio::piped())
                     .output()
                     .expect("failed to execute process");
    let output = String::from_utf8(output1.stdout).unwrap();
    let locind = output.find(" connected primary ");
    if locind != None {
        let start = locind.unwrap() + 18;
        let end = start + 20;
        let getseg1 = output.get(start..end).expect("REASON").to_string();
        let locx = getseg1.find("x");
        if locx != None {
            let end1 = locx.unwrap();
            let start2 = end1 + 1;
            let locy = getseg1.find("+");
            if locy != None {
                let end2 = locy.unwrap();
                let widths = getseg1.get(1..end1);
                let heights = getseg1.get(start2..end2);
                let widths_int: i32 = widths.unwrap().parse().unwrap_or(-99);
                let heights_int: i32 = heights.unwrap().parse().unwrap_or(-99);
                if widths_int > 20 {
                    if heights_int > 75 {
                        widtho = widths_int as u32;
                        heighto = heights_int as u32;
                        errstring = format!("screen size {} x {}", widtho, heighto);
                        errcode = 0;
                    } else {
                        errstring = format!("Invalid height of {:?} -- {}", heights, getseg1);
                        errcode = 1;
                    }
                } else {
                    errstring = format!("Invalid width size {:?} x {:?} -- {}", widths, heights, getseg1);
                    errcode = 2;
                }
            } else {
                errstring = format!("Invalid segment- no + {}", getseg1);
                errcode = 3;
            }
        } else {
            errstring = format!("Invalid segment- no x {}", getseg1);
            errcode = 4;
        }
    } else {
        errstring = format!("Invalid output - no current {:?}", output.get(1..30));
        errcode = 4;
    }
    (errcode, errstring, widtho, heighto)
}

