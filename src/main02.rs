use iced::widget::{button, column, row, text_input, text, horizontal_space, progress_bar};
use iced::{Alignment, Element, Command, Application, Settings, Color};
use iced::theme::{self, Theme};
use iced::executor;
use iced::window;
use iced_futures::futures;
use futures::channel::mpsc;
extern crate chrono;
use std::path::Path;
use std::io::Write;
use std::fs::File;
use std::time::Duration as timeDuration;
use std::thread::sleep;
use chrono::prelude::*;
extern crate walkdir;
use walkdir::WalkDir;

mod get_winsize;
mod inputpress;
mod execpress;
mod findmd5sum;
use get_winsize::get_winsize;
use inputpress::inputpress;
use execpress::execpress;
use findmd5sum::findmd5sum;

pub fn main() -> iced::Result {

     let mut widthxx: u32 = 1350;
     let mut heightxx: u32 = 750;
     let (errcode, errstring, widtho, heighto) = get_winsize();
     if errcode == 0 {
         widthxx = widtho - 20;
         heightxx = heighto - 75;
         println!("{}", errstring);
     } else {
         println!("**ERROR {} get_winsize: {}", errcode, errstring);
     }

     Hdmd5sum::run(Settings {
        window: window::Settings {
            size: (widthxx, heightxx),
            ..window::Settings::default()
        },
        ..Settings::default()
     })
}

struct Hdmd5sum {
    hddir: String,
    mess_color: Color,
    msg_value: String,
    refname: String,
    targetdir: String,
    targetname: String,
    do_progress: bool,
    progval: f64,
    tx_send: mpsc::UnboundedSender<String>,
    rx_receive: mpsc::UnboundedReceiver<String>,
}

#[derive(Debug, Clone)]
enum Message {
    HddirPressed,
    TargetdirPressed,
    RefnameChanged(String),
    TargetnameChanged(String),
    ExecPressed,
    ExecxFound(Result<Execx, Error>),
    ProgressPressed,
    ProgRtn(Result<Progstart, Error>),
}

impl Application for Hdmd5sum {
    type Message = Message;
    type Theme = Theme;
    type Flags = ();
    type Executor = executor::Default;
    fn new(_flags: Self::Flags) -> (Hdmd5sum, iced::Command<Message>) {
        let (tx_send, rx_receive) = mpsc::unbounded();
//        let mut heightxx: f32 = 190.0;
//        let (errcode, errstring, _widtho, heighto) = get_winsize();
//        if errcode == 0 {
//            heightxx = 190.0 + ((heighto as f32 - 768.0) / 2.0);
//            println!("{}", errstring);
//        } else {
//         println!("**ERROR {} get_winsize: {}", errcode, errstring);
//        }
        ( Self { hddir: "--".to_string(), msg_value: "no message".to_string(), targetdir: "--".to_string(),
               mess_color: Color::from([0.0, 0.0, 0.0]), refname: "--".to_string(), 
               targetname: "--".to_string(), do_progress: false, progval: 0.0, tx_send, rx_receive,
 
          },
          Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("Harddrive file list with md5sum -- iced")
    }

    fn update(&mut self, message: Message) -> Command<Message>  {
        match message {
            Message::HddirPressed => {
               let mut inputstr: String = self.hddir.clone();
               if !Path::new(&inputstr).exists() {
                   if Path::new(&self.targetdir).exists() {
                       inputstr = self.targetdir.clone();
                   }
               }
               let (errcode, errstr, newinput) = inputpress(inputstr);
               self.msg_value = errstr.to_string();
               if errcode == 0 {
                   self.hddir = newinput;
               } else {
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
               }
               Command::none()
           }
            Message::RefnameChanged(value) => { self.refname = value; Command::none() }
            Message::TargetnameChanged(value) => { self.targetname = value; Command::none() }
            Message::TargetdirPressed => {
               let mut inputstr: String = self.targetdir.clone();
               if !Path::new(&inputstr).exists() {
                   if Path::new(&self.hddir).exists() {
                       inputstr = self.hddir.clone();
                   }
               }
               let (errcode, errstr, newinput) = inputpress(inputstr);
               self.msg_value = errstr.to_string();
               if errcode == 0 {
                   self.targetdir = newinput.to_string();
                   self.mess_color = Color::from([0.0, 1.0, 0.0]);
               } else {
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
               }
               Command::none()
            }
            Message::ExecPressed => {
               let (errcode, errstr) = execpress(self.hddir.clone(), self.targetdir.clone(), self.refname.clone(), self.targetname.clone());
               self.msg_value = errstr.to_string();
               if errcode == 0 {
                   self.mess_color = Color::from([0.0, 1.0, 0.0]);
                   Command::perform(Execx::execit(self.hddir.clone(),self.targetdir.clone(), self.refname.clone(), self.targetname.clone(), self.tx_send.clone()), Message::ExecxFound)

               } else {
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
                   Command::none()
               }
            }
            Message::ExecxFound(Ok(exx)) => {
               self.msg_value = exx.errval.clone();
               if exx.errcd == 0 {
                   self.mess_color = Color::from([0.0, 1.0, 0.0]);
               } else {
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
               }
               Command::none()
            }
            Message::ExecxFound(Err(_error)) => {
               self.msg_value = "error in copyx copyit routine".to_string();
               self.mess_color = Color::from([1.0, 0.0, 0.0]);
               Command::none()
            }
            Message::ProgressPressed => {
                   self.do_progress = true;
                   Command::perform(Progstart::pstart(), Message::ProgRtn)
            }
            Message::ProgRtn(Ok(_prx)) => {
              if self.do_progress {
                let mut inputval  = " ".to_string();
                let mut bgotmesg = false;
                let mut b100 = false;
                while let Ok(Some(input)) = self.rx_receive.try_next() {
                   inputval = input;
                   bgotmesg = true;
                }
                if bgotmesg {
                    let progvec: Vec<&str> = inputval[0..].split("|").collect();
                    let lenpg1 = progvec.len();
                    if lenpg1 == 3 {
                        let prog1 = progvec[0].to_string();
                        if prog1 == "Progress" {
                            let num_flt: f64 = progvec[1].parse().unwrap_or(-9999.0);
                            if num_flt < 0.0 {
                                println!("progress numeric not numeric: {}", inputval);
                            } else {
                                let dem_flt: f64 = progvec[2].parse().unwrap_or(-9999.0);
                                if dem_flt < 0.0 {
                                    println!("progress numeric not numeric: {}", inputval);
                                } else {
                                    self.progval = 100.0 * (num_flt / dem_flt);
                                    if dem_flt == num_flt {
                                        b100 = true;
                                    } else {
                                        self.msg_value = format!("Convert progress: {:.3}gb of {:.3}gb", (num_flt/1000000000.0), (dem_flt/1000000000.0));
                                        self.mess_color = Color::from([0.0, 0.0, 1.0]);
                                    }
                                }
                            }
                        } else {
                            println!("message not progress: {}", inputval);
                        }
                    } else {
                        println!("message not progress: {}", inputval);
                    }
                } 
                if b100 {
                    Command::none()   
                } else {         
                    Command::perform(Progstart::pstart(), Message::ProgRtn)
                }
              } else {
                Command::none()
              }
            }
            Message::ProgRtn(Err(_error)) => {
                self.msg_value = "error in Progstart::pstart routine".to_string();
                self.mess_color = Color::from([1.0, 0.0, 0.0]);
               Command::none()
            }

        }
    }

    fn view(&self) -> Element<Message> {
        column![
            row![text("Message:").size(20),
                 text(&self.msg_value).size(30).style(*&self.mess_color),
            ].align_items(Alignment::Center).spacing(10).padding(10),
            row![button("Hard drive directory Button").on_press(Message::HddirPressed).style(theme::Button::Secondary),
                 text(&self.hddir).size(20).width(1000)
            ].align_items(Alignment::Center).spacing(10).padding(10),
            row![text("List Reference name: "),
                 text_input("No input....", &self.refname)
                            .on_input(Message::RefnameChanged).padding(10).size(20),
            ].align_items(Alignment::Center).spacing(10).padding(10),
            row![button("Target directory Button").on_press(Message::TargetdirPressed).style(theme::Button::Secondary),
                 text(&self.targetdir).size(20).width(1000)
            ].align_items(Alignment::Center).spacing(10).padding(10),
            row![text("Target file name: "),
                 text_input(".hdlist", &self.targetname)
                            .on_input(Message::TargetnameChanged).padding(10).size(20),
            ].align_items(Alignment::Center).spacing(10).padding(10),
            row![horizontal_space(200),
                 button("Exec Button").on_press(Message::ExecPressed),
            ].align_items(Alignment::Center).spacing(10).padding(10),
            row![button("Start Progress Button").on_press(Message::ProgressPressed),
                 progress_bar(0.0..=100.0,self.progval as f32),
                 text(format!("{:.2}%", &self.progval)).size(30),
            ].align_items(Alignment::Center).spacing(5).padding(10),
         ]
        .padding(5)
        .align_items(Alignment::Start)
        .into()
    }

    fn theme(&self) -> Theme {
       Theme::Dark
/*          Theme::custom(theme::Palette {
                        background: Color::from_rgb8(240, 240, 240),
                        text: Color::BLACK,
                        primary: Color::from_rgb8(230, 230, 230),
                        success: Color::from_rgb(0.0, 1.0, 0.0),
                        danger: Color::from_rgb(1.0, 0.0, 0.0),
                    })
*/               
    }
}

#[derive(Debug, Clone)]
struct Execx {
    errcd: u32,
    errval: String,
}

impl Execx {
//    const TOTAL: u16 = 807;

    async fn execit(hddir: String, targetdir: String, refname: String,  targetname: String, tx_send: mpsc::UnboundedSender<String>,) -> Result<Execx, Error> {
     let errstring  = "Complete harddrive listing".to_string();
     let errcode: u32 = 0;
     let mut linenum: u64 = 0;
     let mut szaccum: u64 = 0;
     let mut numrows: u64 = 0;
     let mut totalsz: u64 = 0;
     let targetfullname: String = format!("{}/{}", targetdir, targetname);
     let mut targetfile = File::create(targetfullname).unwrap();
     for entryx in WalkDir::new(&hddir).into_iter().filter_map(|e| e.ok()) {
          if let Ok(metadata) = entryx.metadata() {
              if metadata.is_file() {
                  numrows = numrows + 1;
                  let file_lenx: u64 = metadata.len();
                  totalsz = totalsz + file_lenx;
              }
          }
     }
     for entry in WalkDir::new(&hddir).into_iter().filter_map(|e| e.ok()) {
          if let Ok(metadata) = entry.metadata() {
              if metadata.is_file() {
                  let fullpath = format!("{}",entry.path().display());
//                  let (errcod, errstr, md5sumv) = findmd5sum(fullpath.clone());
                  let md5sumv = findmd5sum(fullpath.clone());
                  let lrperpos = fullpath.rfind("/").unwrap();
         		  let file_name = fullpath.get((lrperpos+1)..).unwrap();
         		  let file_dir = fullpath.get(0..(lrperpos)).unwrap();
                  let datetime: DateTime<Local> = metadata.modified().unwrap().into();
                  let file_date = format!("{}.000", datetime.format("%Y-%m-%d %T")); 
                  let file_len: u64 = metadata.len();
                  let stroutput = format!("{}|{}|{}|{}|{}|{}",
                                                  file_name,
                                                  file_len,
                                                  file_date,
                                                  file_dir,
                                                  refname,
                                                  md5sumv);
                  writeln!(&mut targetfile, "{}", stroutput).unwrap();
                  linenum = linenum + 1;
                  szaccum = szaccum + file_len;
                  let msgx = format!("Progress|{}|{}", szaccum, totalsz);
                  tx_send.unbounded_send(msgx).unwrap();
              }
          }
     }
     let msgx = format!("Progress|{}|{}", numrows, numrows);
     tx_send.unbounded_send(msgx).unwrap();
     Ok(Execx {
            errcd: errcode,
            errval: errstring,
        })
    }
}
#[derive(Debug, Clone)]
pub enum Error {
//    APIError,
//    LanguageError,
}

// loop thru by sleeping for 5 seconds
#[derive(Debug, Clone)]
pub struct Progstart {
//    errcolor: Color,
//    errval: String,
}

impl Progstart {

    pub async fn pstart() -> Result<Progstart, Error> {
//     let errstring  = " ".to_string();
//     let colorx = Color::from([0.0, 0.0, 0.0]);
     sleep(timeDuration::from_secs(5));
     Ok(Progstart {
//            errcolor: colorx,
//            errval: errstring,
        })
    }
}
