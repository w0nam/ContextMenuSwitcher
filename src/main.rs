use std::{io, process::exit};
use windows_version::*;
use winreg::*;

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
        w10_context_menu()?;
    } else if num == 2 {
        //func here
        version_checker()?;
        key_checker()?;
        revert_to_w11()?;
    } else if num == 0 {
        //exit
        exit(0);
    }
    Ok(())
}

fn revert_to_w11() -> io::Result<()> {
    HKCU.delete_subkey_all("Software\\Classes\\CLSID\\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}")?;
    Ok(())
}

fn w10_context_menu() -> io::Result<()> {
    HKCU.create_subkey(
        "Software\\Classes\\CLSID\\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\\InprocServer32",
    )?;
    Ok(())
}

fn key_checker() -> io::Result<()> {
    // I'll have to refactor this function later on.
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
    user_choice()?;
    Ok(())
}
