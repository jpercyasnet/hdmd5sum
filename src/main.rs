use iced::widget::{button, column, row, text_input, text, Space, progress_bar};
use iced::{Alignment, Element, Task, Color};
use iced::theme::{Theme};
use iced::futures;
use futures::channel::mpsc;
extern crate chrono;
use std::path::Path;
use std::io::Write;
use std::fs::File;
use std::time::Duration as timeDuration;
use std::time::Instant as timeInstant;
use std::thread::sleep;
use chrono::prelude::*;
use chrono::Local;

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

     let mut widthxx: f32 = 1350.0;
     let mut heightxx: f32 = 750.0;
     let (errcode, errstring, widtho, heighto) = get_winsize();
     if errcode == 0 {
         widthxx = widtho as f32 - 20.0;
         heightxx = heighto as f32 - 75.0;
         println!("{}", errstring);
     } else {
         println!("**ERROR {} get_winsize: {}", errcode, errstring);
     }
     iced::application(Hdmd5sum::title, Hdmd5sum::update, Hdmd5sum::view)
        .window_size((widthxx, heightxx))
        .theme(Hdmd5sum::theme)
        .run_with(Hdmd5sum::new)
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

impl Hdmd5sum {
    fn new() -> (Hdmd5sum, iced::Task<Message>) {
        let (tx_send, rx_receive) = mpsc::unbounded();
        ( Self { hddir: "--".to_string(), msg_value: "no message".to_string(), targetdir: "--".to_string(),
               mess_color: Color::from([0.0, 0.0, 1.0]), refname: "--".to_string(), 
               targetname: "--".to_string(), do_progress: false, progval: 0.0, tx_send, rx_receive,
 
          },
          Task::none()
        )
    }

    fn title(&self) -> String {
        String::from("Harddrive file list with md5sum -- iced")
    }

    fn update(&mut self, message: Message) -> Task<Message>  {
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
               Task::none()
           }
            Message::RefnameChanged(value) => { self.refname = value; Task::none() }
            Message::TargetnameChanged(value) => { self.targetname = value; Task::none() }
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
               Task::none()
            }
            Message::ExecPressed => {
               let (errcode, errstr) = execpress(self.hddir.clone(), self.targetdir.clone(), self.refname.clone(), self.targetname.clone());
               self.msg_value = errstr.to_string();
               if errcode == 0 {
                   self.mess_color = Color::from([0.0, 1.0, 0.0]);
                   Task::perform(Execx::execit(self.hddir.clone(),self.targetdir.clone(), self.refname.clone(), self.targetname.clone(), self.tx_send.clone()), Message::ExecxFound)

               } else {
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
                   Task::none()
               }
            }
            Message::ExecxFound(Ok(exx)) => {
               self.msg_value = exx.errval.clone();
               if exx.errcd == 0 {
                   self.mess_color = Color::from([0.0, 1.0, 0.0]);
               } else {
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
               }
               Task::none()
            }
            Message::ExecxFound(Err(_error)) => {
               self.msg_value = "error in copyx copyit routine".to_string();
               self.mess_color = Color::from([1.0, 0.0, 0.0]);
               Task::none()
            }
            Message::ProgressPressed => {
                   self.do_progress = true;
                   Task::perform(Progstart::pstart(), Message::ProgRtn)
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
                    if lenpg1 == 4 {
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
                                        self.msg_value = format!("md5sum progress: {:.3}gb of {:.3}gb {}", (num_flt/1000000000.0), (dem_flt/1000000000.0), progvec[3]);
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
                    Task::none()   
                } else {         
                    Task::perform(Progstart::pstart(), Message::ProgRtn)
                }
              } else {
                Task::none()
              }
            }
            Message::ProgRtn(Err(_error)) => {
                self.msg_value = "error in Progstart::pstart routine".to_string();
                self.mess_color = Color::from([1.0, 0.0, 0.0]);
               Task::none()
            }

        }
    }

    fn view(&self) -> Element<Message> {
        column![
            row![text("Message:").size(20),
                 text(&self.msg_value).size(30).color(*&self.mess_color),
            ].align_y(Alignment::Center).spacing(10).padding(10),
            row![button("Hard drive directory Button").on_press(Message::HddirPressed),
                 text(&self.hddir).size(20).width(1000)
            ].align_y(Alignment::Center).spacing(10).padding(10),
            row![text("List Reference name: "),
                 text_input("No input....", &self.refname)
                            .on_input(Message::RefnameChanged).padding(10).size(20),
            ].align_y(Alignment::Center).spacing(10).padding(10),
            row![button("Target directory Button").on_press(Message::TargetdirPressed),
                 text(&self.targetdir).size(20).width(1000)
            ].align_y(Alignment::Center).spacing(10).padding(10),
            row![text("Target file name: "),
                 text_input(".hdlist", &self.targetname)
                            .on_input(Message::TargetnameChanged).padding(10).size(20),
            ].align_y(Alignment::Center).spacing(10).padding(10),
            row![Space::with_width(200),
                 button("Exec Button").on_press(Message::ExecPressed),
            ].align_y(Alignment::Center).spacing(10).padding(10),
            row![button("Start Progress Button").on_press(Message::ProgressPressed),
                 progress_bar(0.0..=100.0,self.progval as f32),
                 text(format!("{:.2}%", &self.progval)).size(30),
            ].align_y(Alignment::Center).spacing(5).padding(10),
         ]
        .padding(5)
        .align_x(Alignment::Start)
        .into()
    }

    fn theme(&self) -> Theme {
       Theme::Dracula
    }
}

#[derive(Debug, Clone)]
struct Execx {
    errcd: u32,
    errval: String,
}

impl Execx {

    async fn execit(hddir: String, targetdir: String, refname: String,  targetname: String, tx_send: mpsc::UnboundedSender<String>,) -> Result<Execx, Error> {
     let mut errstring  = "Complete harddrive listing".to_string();
     let mut errcode: u32 = 0;
     let mut linenum: u64 = 0;
     let mut szaccum: u64 = 0;
     let mut numrows: u64 = 0;
     let mut xrows: u64 = 1000;
     let mut totalsz: u64 = 0;
     let start_time = timeInstant::now();
     let targetfullname: String = format!("{}/{}", targetdir, targetname);
     let mut targetfile = File::create(targetfullname).unwrap();
     for entryx in WalkDir::new(&hddir).into_iter().filter_map(|e| e.ok()) {
          if let Ok(metadata) = entryx.metadata() {
              if metadata.is_file() {
                  numrows = numrows + 1;
                  if numrows > xrows {
                     let datenn = Local::now();
                     let msgn = format!("Progress|0|100| at {} for {} files", datenn.format("%H:%M:%S"), numrows);
                     tx_send.unbounded_send(msgn).unwrap();
                     xrows = xrows + 1000;
                  }
                  let file_lenx: u64 = metadata.len();
                  totalsz = totalsz + file_lenx;
              }
          }
     }
     if numrows < 1 {
         errstring  = "no files on disk".to_string();
         errcode = 1;
     } else {
        let diffy = start_time.elapsed();
        let minsy: f64 = diffy.as_secs() as f64/60 as f64;
        let dateyy = Local::now();
        let msgx = format!("Progress|{}|{}| elapsed time {:.1} mins at {} for {} files", szaccum, totalsz, minsy, dateyy.format("%H:%M:%S"), numrows);
        tx_send.unbounded_send(msgx).unwrap();
        for entry in WalkDir::new(&hddir).into_iter().filter_map(|e| e.ok()) {
          if let Ok(metadata) = entry.metadata() {
              if metadata.is_file() {
                  let fullpath = format!("{}",entry.path().display());
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
                  let diffx = start_time.elapsed();
                  let minsx: f64 = diffx.as_secs() as f64/60 as f64;
                  let datexx = Local::now();
                  let msgx = format!("Progress|{}|{}| elapsed time {:.1} mins at {} {} of {}", szaccum, totalsz, minsx, datexx.format("%H:%M:%S"), linenum, numrows);
                  tx_send.unbounded_send(msgx).unwrap();
              }
          }
        }
     }
     let msgx = format!("Progress|{}|{}| end of md5sum process", numrows, numrows);
     tx_send.unbounded_send(msgx).unwrap();
     Ok(Execx {
            errcd: errcode,
            errval: errstring,
        })
    }
}
#[derive(Debug, Clone)]
pub enum Error {
}

// loop thru by sleeping for 5 seconds
#[derive(Debug, Clone)]
pub struct Progstart {
}

impl Progstart {

    pub async fn pstart() -> Result<Progstart, Error> {
     sleep(timeDuration::from_secs(5));
     Ok(Progstart {
        })
    }
}
