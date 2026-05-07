use std::io;
use windows_version::*;
use winreg::*;

const WIN_11_MIN_VERSION: u32 = 22000; // Lowest build number of W11, lower than that would be W10

fn menu_switcher() {
    println!("Menu switcher fun achieved!");
}

fn key_checker() -> io::Result<()> {
    HKCU.open_subkey("Software\\Classes\\CLSID\\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}")?;
    Ok(())
}

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
    key_checker()?;

    println!(
        "RegEdit Context Menu Key already present, would you like to revert it to W11 context menu ? (y/N)"
    );
    // Forgor how to register user input in rust...
    // let usr_input = String::new();
    // println!("{}", usr_input);
    menu_switcher();
    Ok(())
}
