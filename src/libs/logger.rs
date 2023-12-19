use std::fs::File;
use chrono::Local;
use log::{info, LevelFilter};
use std::io::Write;



pub fn init_file_logger(){
    let log_file: Box<File> = Box::new(File::create("hard-sync.log").expect("Can't create file"));
    env_logger::Builder::new()
    .target(env_logger::Target::Pipe(log_file))
    .filter(None, LevelFilter::Debug)
    .format(|buf, record| {
        writeln!(
            buf,
            "[{} {} {}:{}] {}",
            Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
            record.level(),
            record.file().unwrap_or("unknown"),
            record.line().unwrap_or(0),
            record.args()
        )
    })
    .init();
}

pub fn init_console_logger(){
    env_logger::Builder::from_default_env()
    .filter(None, LevelFilter::Debug)
    .write_style(env_logger::WriteStyle::Always)
    .target(env_logger::Target::Stdout)
    .init();
}

pub fn init(){
    // if development use console logger else use file logger
    if cfg!(debug_assertions) {
        println!("debug mode");
        init_console_logger();
    } else {
        init_file_logger();
    }
}