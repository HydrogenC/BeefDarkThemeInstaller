extern crate resource;
extern crate winapi;

use winapi::um::processenv::*;
use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::OsStrExt;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use std::fs;
use std::str::FromStr;
use std::io::stdin;

fn to_lpcwstr(s: &OsStr) -> Vec<u16> {
    let mut vc: Vec<u16> = s.encode_wide().collect();
    vc.push(0);
    vc
}

fn main() {
    unsafe {
        let env_name = OsStr::new("BeefPath");
        let mut path = [0u16; 260];

        let len = GetEnvironmentVariableW(to_lpcwstr(env_name).as_ptr(), path.as_mut_ptr(), 260);
        let str: OsString = OsString::from_wide(&mut path[0..len as usize]);
        println!("Detected Beef at {}", str.to_str().unwrap());
        println!("Press enter to install...");
        {
            let mut not_used=String::new();
            stdin().read_line(&mut not_used);
        }
        let mut pbuf = PathBuf::from_str(str.to_str().unwrap())
            .unwrap()
            .join("bin")
            .join("themes")
            .join("SimplyDark");

        if pbuf.exists() {
            println!("Theme already exists, replacing...");
        }else{
            fs::create_dir(&pbuf);
        }

        let bytes_1 = resource::resource!("files/UI_4.psd").to_vec();
        fs::write(pbuf.join("UI_4.psd"), bytes_1);
        let bytes_2 = resource::resource!("files/theme.toml").to_vec();
        fs::write(pbuf.join("theme.toml"), bytes_2);

        println!("Success! ");
        println!("Press enter to exit...");
    }
}
