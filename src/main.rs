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
// Used '&mut String' instead of '&mut str' was throwing an error otherwise.
fn user_choice(input: &mut String) -> io::Result<MenuChoices> {
    loop {
        println!(
            r#"// WINDOWS 11 CONTEXT MENU SWITCHER

(PSA: the program will restart explorer.exe for the patch to be applied on-the-fly, screen flashing is normal.)

Please select an option:
    1) Deploy Windows 10 Context Menu.
    2) Revert to Windows 11 Context Menu.
    0) Exit and do nothing.

Enter your choice:"#
        );
        io::stdin().read_line(input)?;

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
    // Forward the last result
    restart_explorer()
}

fn w10_menu_style() -> io::Result<()> {
    Command::new("REG.exe")
        .args(["add",
            "HKCU\\Software\\Classes\\CLSID\\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\\InprocServer32",
            "/f",
            "/ve"])
        .status()?;
    // Forward the last result
    restart_explorer()
}

fn key_checker() -> io::Result<bool> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Err(e) =
        hkcu.open_subkey("Software\\Classes\\CLSID\\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}")
    {
        return match e.kind() {
            io::ErrorKind::NotFound => Ok(false),
            _ => Err(e),
        };
    }

    Ok(true)
}

fn version_checker() -> io::Result<()> {
    if OsVersion::current().build <= WIN_11_MIN_VERSION {
        return Err(io::Error::other(
            "Not running Windows 11: no need to change the context menu. Aborting...",
        ));
    }

    Ok(())
}

fn main() -> io::Result<()> {
    version_checker()?;
    let mut input = String::with_capacity(1);
    match user_choice(&mut input)? {
        MenuChoices::DeployW10 => {
            if key_checker()? {
                println!("Patch already applied!");
            } else {
                w10_menu_style()?;
            }
        }
        MenuChoices::DeployW11 => {
            w11_menu_style()?;
        }
        MenuChoices::Exit => {
            // Added some text when the user want to quit out.
            println!("Alright, bye-bye!");
            std::thread::sleep(std::time::Duration::from_millis(500));
            return Ok(());
        }
    }

    println!("Press ENTER to exit...");
    io::stdin().read_line(&mut input)?;

    Ok(())
}
