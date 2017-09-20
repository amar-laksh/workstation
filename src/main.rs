#[warn(unreachable_code)]
#[macro_use]
extern crate shells;
extern crate opencv;
extern crate clap;
use clap::{Arg, App};
use opencv::*;
use std::time::Instant;
use std::time::Duration;

fn run(rect: i32, timeout: i64) -> Result<(),String> {
    let xml = "./haarcascade_frontalface_alt.xml";
    let mut seen = 0;
    let mut area;
    loop {
        let mut cam = try!(highgui::VideoCapture::device(0));
        let mut face = try!(objdetect::CascadeClassifier::new(xml));
        loop {
            let mut frame = try!(core::Mat::new());
            try!(cam.read(&mut frame));
            if try!(frame.size()).width == 0 {
                ::std::thread::sleep(Duration::from_millis(50));
                continue;
            }
            let mut gray = try!(core::Mat::new());
            try!(imgproc::cvt_color(&frame
                                    , &mut gray
                                    , imgproc::CV_BGR2GRAY, 0));
            let mut reduced = try!(core::Mat::new());
            try!(imgproc::resize(&gray
                                 , &mut reduced
                                 , core::Size{width:0,height:0}
                                 , 0.25f64
                                 , 0.25f64
                                 , imgproc::INTER_LINEAR));
            let mut faces = ::opencv::types::VectorOfRect::new();
            try!(face.detect_multi_scale(
                    &reduced
                    , &mut faces, 1.1, 2
                    , objdetect::CV_HAAR_SCALE_IMAGE
                    , core::Size{ width:30, height:30 }
                    , core::Size{ width:0, height:0 }));
            if faces.len() != 0 {
                area = faces[0].width * faces[0].height;
                // println!("Area: {:?}, rect: {:?}", area, rect);
                if area >= rect {
                    sh!(r#"echo 0 >
                        /sys/class/backlight
                        /intel_backlight/brightness"#);
                    seen = 1;
                } else if area < rect && seen == 1 {
                    sh!(r#"echo 937 >
                        /sys/class/backlight
                        /intel_backlight/brightness"#);
                }
            } else {
                // println!("NO FACE FOUND...WAITING TO LOCK.");
                let now = Instant::now();
                let mut elapsed;
                let mut sec = 0;
                let mut still_faces =
                    ::opencv::types::VectorOfRect::new();
                let mut saved = 0;
                while  sec <= timeout {
                    elapsed = now.elapsed();
                    sec = (
                        (elapsed.as_secs() as f64)
                        + (elapsed.subsec_nanos() as f64
                           / 1000_000_000.0)
                        ) as i64;
                    let mut frame = try!(core::Mat::new());
                    try!(cam.read(&mut frame));
                    if try!(frame.size()).width == 0 {
                        ::std::thread::sleep(
                            Duration::from_millis(50)
                            );
                        continue;
                    }
                    let mut gray = try!(core::Mat::new());
                    try!(imgproc::cvt_color(
                            &frame
                            , &mut gray
                            , imgproc::CV_BGR2GRAY, 0));
                    let mut reduced = try!(core::Mat::new());
                    try!(imgproc::resize(
                            &gray
                            , &mut reduced
                            , core::Size{width:0,height:0}
                            , 0.25f64, 0.25f64
                            , imgproc::INTER_LINEAR));
                    still_faces =
                        ::opencv::types::VectorOfRect::new();
                    try!(face.detect_multi_scale(
                            &reduced
                            , &mut still_faces, 1.1, 2
                            , objdetect::CV_HAAR_SCALE_IMAGE
                            , core::Size{ width:30, height:30 }
                            , core::Size{ width:0, height:0 }));
                    if still_faces.len() != 0 {
                        saved = 1;
                        sh!(r#"echo 937 >
                            /sys/class/backlight
                            /intel_backlight/brightness"#);
                        break;
                    }
                }
                if saved == 1 {
                    continue;
                }
                else {
                    if still_faces.len() == 0 {
                        sh!("xdotool key Super+l");
                    }
                }
            }
        }
    }
    Ok(())
}

fn main() {
    let (_, stdout, _) = sh!("echo $EUID");
    if stdout != "0\n" {
        println!("Please run this command as root.");
        return;
    }
    let matches = App::new("workstation")
        .version("0.1.0")
        .author("Amar L. <amar.lakshya@xaviers.edu.in>")
        .about(r#"helps you at the workstation by keeping you
               far from screen and locks system when you are
               away, among other things."#)
        .arg(Arg::with_name("rect")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("size of rectangle for screen dimming"))
        .arg(Arg::with_name("timeout")
                 .required(true)
                 .takes_value(true)
                 .index(2)
                 .help("timeout for locking the desktop"))
        .get_matches();
    let rect = matches.value_of("rect")
        .unwrap().parse::<i32>().unwrap();
    let timeout = matches.value_of("timeout")
        .unwrap().parse::<i64>().unwrap();
    run(rect, timeout).unwrap();
}
