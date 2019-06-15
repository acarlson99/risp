use std::fs::OpenOptions;
use std::io::prelude::*;
use std::process::Command;
use std::str;

const REPL0: &str = "
██████╗ ██╗███████╗██████╗
██╔══██╗██║██╔════╝██╔══██╗
██████╔╝██║███████╗██████╔╝
██╔══██╗██║╚════██║██╔═══╝
██║  ██║██║███████║██║ REPL
╚═╝  ╚═╝╚═╝╚══════╝╚═╝ ";
const REPL1: &str = "Use Ctrl-C or Ctrl-D to exit REPL";

fn main() -> std::io::Result<()> {
    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()?;
    if output.status.success() {
        let mut res = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(".repl_logo")?;
        let out = str::from_utf8(&output.stdout).unwrap();
        let txt = format!("{}{}{}", REPL0, out, REPL1);
        res.write_all(txt.as_bytes())?;
    }
    Ok(())
}
