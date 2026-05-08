use std::{io, process::Command, *};
use windows_version::*;
use winreg::{RegKey, enums::HKEY_CURRENT_USER};

const WIN_11_MIN_VERSION: u32 = 22000; // Lowest build number of W11, lower than that would be W10

enum MenuChoices {
    DeployW10,
    DeployW11,
    Exit,
}

#[cfg(target_os = "windows")]
fn user_choice() -> io::Result<MenuChoices> {
    loop {
        println!("Windows 11 Context Menu Switcher");
        println!(
            "(the program will restart explorer.exe for the patch to be applied on-the-fly, screen flashing is normal.)"
        );
        println!(
            "Please select an option:\n\n1) Deploy Windows 10 Context Menu.\t2) Revert to Windows 11 Context Menu.\t0) Exit and do nothing.\n\n"
        );
        println!("Enter your choice:");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "1" => return Ok(MenuChoices::DeployW10),
            "2" => return Ok(MenuChoices::DeployW11),
            "0" => return Ok(MenuChoices::Exit),
            _ => {
                println!("Invalid input, try again.");
                continue;
            }
        }
    }
}

fn restart_explorer() -> io::Result<()> {
    Command::new("taskkill")
        .args(["/F", "/IM", "explorer.exe"])
        .status()?;

    std::thread::sleep(std::time::Duration::from_millis(500));

    Command::new("explorer.exe").spawn()?;

    Ok(())
}

fn w11_menu_style() -> io::Result<()> {
    Command::new("REG.exe")
        .args([
            "delete",
            "HKCU\\Software\\Classes\\CLSID\\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}",
            "/f",
        ])
        .status()?;

    restart_explorer()?;

    Ok(())
}

fn w10_menu_style() -> io::Result<()> {
    Command::new("REG.exe")
        .args(["add",
            "HKCU\\Software\\Classes\\CLSID\\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\\InprocServer32",
            "/f",
            "/ve"])
        .status()?;

    restart_explorer()?;

    Ok(())
}

fn key_checker() -> io::Result<bool> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    match hkcu.open_subkey("Software\\Classes\\CLSID\\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}") {
        Ok(_) => Ok(true),
        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
                Ok(false)
            } else {
                Err(e)
            }
        }
    }
}

fn version_checker() -> io::Result<()> {
    if OsVersion::current().build <= WIN_11_MIN_VERSION {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Not running Windows11: no need to change the context menu. Aborting...",
        ));
    } else {
        return Ok(());
    }
}

fn main() -> io::Result<()> {
    version_checker()?;
    match user_choice()? {
        MenuChoices::DeployW10 => {
            if key_checker()? {
                println!("Patch already applied!");
            } else {
                w10_menu_style()?;
            }
        }
        MenuChoices::DeployW11 => {
            //do something;
            w11_menu_style()?;
        }
        MenuChoices::Exit => {
            //do something
            return Ok(());
        }
    }

    println!("Press ENTER to exit...");
    let mut quit = String::new();
    io::stdin().read_line(&mut quit)?;

    Ok(())
}
