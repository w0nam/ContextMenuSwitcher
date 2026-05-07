use std::io;
use windows_version::*;
//use winreg::*;

const WIN_11_MIN_VERSION: u32 = 27000;
// fn menu_switcher() {
//     println!("Menu switcher here!");
// }

// fn key_checker() -> io::Result<()> {
//     HKCU.open_subkey("Software\\Classes\\CLSID\\{86ca1aa0-34aa-4e8b-a509-50c905bae2a3}")?;
// }

fn version_checker() -> io::Result<()> {
    if OsVersion::current().build <= WIN_11_MIN_VERSION {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Not Windows11, no need to change the context menu.",
        ));
    } else {
        return Ok(());
    }
}

fn main() -> io::Result<()> {
    version_checker()?;
    //key_checker()?;

    Ok(())
}
