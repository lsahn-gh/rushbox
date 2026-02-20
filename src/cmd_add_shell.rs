use rand::Rng;
use std::{fs::{self, File}, io::{BufWriter, Write}, path::Path};
use std::error::Error;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                        abcdefghijklmnopqrstuvwxyz\
                        0123456789";
const ADDITIONAL_LEN: usize = 8;

pub fn main(args: &[String]) -> Result<i32, Box<dyn Error>> {
    if args.is_empty() {
        return Err("No arguments provided".into());
    }

    let path = Path::new(&args[0]);
    if path.exists() == false {
        return Err("The specified shell does not exist".into());
    }

    /* generate random tmp path */
    let mut rand = rand::rng();
    let postfix: String = (0..ADDITIONAL_LEN)
        .map(|_| {
            let idx = rand.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    let target = format!("/tmp/shells-{}.tmp", postfix);

    /* append and replace the file */
    fs::copy("/etc/shells", &target)?;
    let mut buf = BufWriter::new(
        File::options()
            .write(true)
            .open(&target)?
    );
    buf.write_all(format!("{}\n", args[0]).as_bytes())?;
    buf.flush()?;
    fs::copy(&target, "/etc/shells")?;
    fs::remove_file(&target)?;

    Ok(0)
}