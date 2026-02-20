use std::{error::Error, fs::{self, OpenOptions}, io::{self, BufRead, BufReader, Write}};
use libc::uid_t;

use crate::util;

const ROOT_UID: uid_t = 0;
const GROUP_FILE: &str = "/etc/group";

pub fn main(args: &[String]) -> Result<i32, Box<dyn Error>> {
    if args.len() != 2 {
        eprintln!("Usage: addgroup <groupname>");
        return Err("Invalid number of arguments".into());
    }

    if util::current_euid() != ROOT_UID {
        return Err("You must be root to add a group".into());
    }

    let group_name = &args[1];
    let file = fs::File::open(GROUP_FILE)?;
    let reader = BufReader::new(file);

    let mut max_gid = 999;

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(':').collect();
        // 빈 줄이나 주석 처리
        if parts.len() < 3 {
            continue;
        }

        let existing_name = parts[0];
        if existing_name == group_name {
            return Err("Group already existed".into());
        }

        let existing_gid = parts[2].parse::<u32>().unwrap_or(0);
        if existing_gid > max_gid && existing_gid < 65534 {
            max_gid = existing_gid;
        }
    }

    // new GID
    let new_gid = max_gid + 1;

    // append the group to the file
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(GROUP_FILE)?;

    let _ = writeln!(file, "{}:x:{}", group_name, new_gid);

    Ok(0)
}