extern crate lazy_concat;

use winreg::RegKey;
use winreg::enums::HKEY_LOCAL_MACHINE;
use std::process::Command;
use std::path::{Path, PathBuf};
use lazy_concat::LazyConcat;

pub struct CpuInfo {
    hklm: RegKey,
}

const OFFICE_PATHS: [&str; 10] = [
    "C:\\Program Files\\Microsoft Office\\Office16",
    "C:\\Program Files (x86)\\Microsoft Office\\Office16", // 2016+ / 365
    "C:\\Program Files\\Microsoft Office\\Office15",
    "C:\\Program Files (x86)\\Microsoft Office\\Office15", // 2013
    "C:\\Program Files\\Microsoft Office\\Office14",
    "C:\\Program Files (x86)\\Microsoft Office\\Office14", // 2010
    "C:\\Program Files\\Microsoft Office\\Office12",
    "C:\\Program Files (x86)\\Microsoft Office\\Office12", // 2007
    "C:\\Program Files\\Microsoft Office\\Office11",
    "C:\\Program Files (x86)\\Microsoft Office\\Office11" // 2003
];

impl CpuInfo {

    pub fn new() -> CpuInfo {
        CpuInfo {hklm: RegKey::predef(HKEY_LOCAL_MACHINE)}
    }

    pub fn compname() -> String {
        // need type annotation
        let cn1: String = CpuInfo::new().hklm.open_subkey("SYSTEM\\CurrentControlSet\\Control\\ComputerName\\ComputerName").unwrap().get_value("ComputerName").unwrap();
        return cn1;
    }

    pub fn windows_name() -> String {
        // need type annotation
        let cn1: String = CpuInfo::new().hklm.open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion").unwrap().get_value("ProductName").unwrap();
        return cn1;
    }

    pub fn system_info() -> String {
        let mut sysinfo = Command::new("systeminfo");
        return sysinfo.output().unwrap().stdout.iter().map(|&c| c as char).collect::<String>();
    }

    pub fn _office_info(folder: &str) -> String {
        let mut officecmd = Command::new("cscript");

        let officeoutput = officecmd
            .current_dir(folder)
            .arg("ospp.vbs")
            .arg("/dstatus")
            .output().unwrap().stdout.iter().map(|&c| c as char).collect::<String>();
        let mut office_version = "Office not found";
        if folder == OFFICE_PATHS[0] || folder == OFFICE_PATHS[1] {
            office_version = "Office 2016 or newer OR Office 365";
        }
        if folder == OFFICE_PATHS[2] || folder == OFFICE_PATHS[3] {
            office_version = "Office 2013";
        }
        if folder == OFFICE_PATHS[4] || folder == OFFICE_PATHS[5] {
            office_version = "Office 2010";
        }
        if folder == OFFICE_PATHS[6] || folder == OFFICE_PATHS[7] {
            office_version = "Office 2007";
        }
        if folder == OFFICE_PATHS[8] || folder == OFFICE_PATHS[9] {
            office_version = "Office 2003";
        }
        return LazyConcat::new(office_version.to_owned()).and_concat("\n\nLICENSE INFORMATION:\n\n").and_concat(&officeoutput).done().to_owned();
    }

    pub fn office_info() -> String {
        for office_path in OFFICE_PATHS.iter() {
            let mut _test_buf = PathBuf::from(office_path);
            _test_buf.push("OSPP.VBS");
            if _test_buf.exists() {
                return CpuInfo::_office_info(office_path);
            }
        }
        return "Office not installed or not installed in default location".to_owned();
    }
}