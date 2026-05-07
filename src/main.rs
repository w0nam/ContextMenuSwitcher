use std::{io, process::Command, process::exit};
use windows_version::*;

const WIN_11_MIN_VERSION: u32 = 22000; // Lowest build number of W11, lower than that would be W10

fn user_choice() -> io::Result<()> {
    println!(
        "Windows 11 Context Menu Switcher\n\nPlease select an option:\n\n1) Deploy Windows 10 Context Menu.\t2) Revert to Windows 11 Context Menu.\t0) Exit and do nothing.\n\n"
    );
    let mut input = String::new();
    println!("Enter your query :");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let num: u32 = input.trim().parse().expect("Invalid input.");
    if num == 1 {
        // func here
        version_checker()?;
        w10_menu_style();
    } else if num == 2 {
        version_checker()?;
        w11_menu_style();
    } else if num == 0 {
        exit(0);
    }
    Ok(())
}

fn w11_menu_style() {
    Command::new("REG.exe")
        .args([
            "delete",
            "HKCU\\Software\\Classes\\CLSID\\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}",
            "/f",
        ])
        .spawn()
        .ok()
        .expect("Failed to execute.");
}

fn w10_menu_style() {
    Command::new("REG.exe")
        .args(["add",
            "HKCU\\Software\\Classes\\CLSID\\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\\InprocServer32",
            "/f",
            "/ve"])
        .spawn()
        .ok()
        .expect("ERROR: Cannot create the key");
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
    user_choice()?;
    Ok(())
}
