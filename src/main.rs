extern crate file;
extern crate lazy_concat;
extern crate winreg;

mod cpuinfo;

use lazy_concat::LazyConcat;
use std::path::{Path, PathBuf};
use winreg::enums::*;
use winreg::RegKey;
use std::fs::create_dir_all;
use std::time::SystemTime;
use crate::cpuinfo::CpuInfo;
use std::process::{Command, exit};
use std::io::{Read, stdin, BufRead};

fn append(path: &str, line: &str) {
    // don't read from file if it doesn't already exist
    if !Path::new(path).exists() {
        file::put_text(path, line);
        return;
    }
    let existing_text = file::get_text(path).unwrap();
    file::put_text(path, LazyConcat::new(existing_text).and_concat("\n").and_concat(line).done());
}

fn append_details(dest_folder: PathBuf, subfilename: &str, line: &str) {
    let mut _dest_folder = dest_folder.clone();
    _dest_folder.push(subfilename);
    append(_dest_folder.to_str().unwrap(), line);
}

fn outfolder() -> PathBuf {
    let mut full_buff = std::env::current_exe().unwrap();
    full_buff.pop();
    full_buff.push("computers");
    return full_buff;
}

fn unix_time_str() -> String {
    return SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs().to_string();
}

fn dest_folder() -> PathBuf {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let mut buff = outfolder();
    let cpu_name = CpuInfo::compname();
    buff.push(LazyConcat::new(cpu_name).and_concat("-").and_concat(unix_time_str()).done());
    std::fs::create_dir_all(buff.clone());
    return buff;
}

fn main() {
    println!("Creating Folder");
    let dest_folder = dest_folder();
    println!("Getting Windows version");
    append_details(dest_folder.clone(), "Windows_version.txt", &CpuInfo::windows_name());
    println!("Gathering windows information");
    append_details(dest_folder.clone(), "systeminfo.txt", &CpuInfo::system_info());
    println!("Gathering office information");
    append_details(dest_folder.clone(), "microsoft_office.txt", &CpuInfo::office_info());
    let mut input = String::new();
    println!("Done!");
    println!("Press enter to exit");
    for line in std::io::stdin().lock().lines() {
        exit(0)
    }
}
