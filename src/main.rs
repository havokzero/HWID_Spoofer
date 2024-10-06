use std::process::{Command, exit};
use std::env;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};
use winreg::enums::*;
use winreg::RegKey;
use colored::*;
use rand::{thread_rng, Rng};
use uuid::Uuid;

// Generate a random alphanumeric string
fn random_id(len: usize) -> String {
    let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
        .chars()
        .collect();
    let mut rng = thread_rng();
    (0..len)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect()
}

// Log and handle errors gracefully instead of panicking
fn handle_result<T>(result: Result<T, &'static str>, success_message: &str) {
    match result {
        Ok(_) => println!("{}", success_message.green()),
        Err(e) => eprintln!("{}", format!("Error: {}", e).red()),
    }
}

// Spoof the Installation ID in the registry
fn spoof_installation_id() -> Result<(), &'static str> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let current_version = hklm.open_subkey_with_flags("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion", KEY_WRITE)
        .map_err(|_| "Failed to open registry key")?;
    let new_id = Uuid::new_v4().to_string();
    current_version.set_value("InstallationID", &new_id).map_err(|_| "Failed to set registry value")?;
    Ok(())
}

// Spoof the PC name in the registry
fn spoof_pc_name() -> Result<(), &'static str> {
    let random_name = random_id(8);
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    let computer_name_key = hklm.open_subkey_with_flags("SYSTEM\\CurrentControlSet\\Control\\ComputerName\\ComputerName", KEY_WRITE)
        .map_err(|_| "Failed to open registry key for PC name")?;
    computer_name_key.set_value("ComputerName", &random_name).map_err(|_| "Failed to set ComputerName")?;

    let active_computer_name_key = hklm.open_subkey_with_flags("SYSTEM\\CurrentControlSet\\Control\\ComputerName\\ActiveComputerName", KEY_WRITE)
        .map_err(|_| "Failed to open registry key for active PC name")?;
    active_computer_name_key.set_value("ComputerName", &random_name).map_err(|_| "Failed to set ActiveComputerName")?;

    Ok(())
}

// Change the PC hostname using the `wmic` command
fn spoof_hostname() -> Result<(), &'static str> {
    let new_hostname = random_id(10);
    println!("{}", format!("Changing hostname to {}", new_hostname).green());

    let change_result = Command::new("cmd")
        .args(&["/C", &format!("wmic computersystem where name=\"%COMPUTERNAME%\" call rename name=\"{}\"", new_hostname)])
        .output()
        .map_err(|_| "Failed to change hostname")?;

    if change_result.status.success() {
        println!("{}", format!("Hostname changed successfully. You need to reboot your computer for the changes to take effect.").green());
    } else {
        return Err("Failed to change hostname");
    }

    Ok(())
}

// Spoof the hardware profile GUID in the registry
fn spoof_guid() -> Result<(), &'static str> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let hw_profile_guid_key = hklm.open_subkey_with_flags("SYSTEM\\CurrentControlSet\\Control\\IDConfigDB\\Hardware Profiles\\0001", KEY_WRITE)
        .map_err(|_| "Failed to open registry key")?;
    let new_guid = Uuid::new_v4().to_string();
    hw_profile_guid_key.set_value("HwProfileGuid", &format!("{{{}}}", new_guid))
        .map_err(|_| "Failed to set HwProfileGuid")?;
    println!("HwProfileGuid spoofed successfully!");
    Ok(())
}

// Spoof the Machine GUID in the registry
fn spoof_machine_guid() -> Result<(), &'static str> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let cryptography_key = hklm.open_subkey_with_flags("SOFTWARE\\Microsoft\\Cryptography", KEY_WRITE)
        .map_err(|_| "Failed to open registry key for machine GUID")?;

    let new_guid = Uuid::new_v4().to_string();
    cryptography_key.set_value("MachineGuid", &new_guid)
        .map_err(|_| "Failed to set new machine GUID")?;

    println!("Machine GUID spoofed successfully!");
    Ok(())
}

// Spoof the MAC address using `netsh`
fn spoof_mac() -> Result<(), &'static str> {
    let new_mac = random_mac();
    println!("{}", format!("Spoofing MAC to {}", new_mac).green());

    let disable_result = Command::new("cmd")
        .args(&["/C", "netsh interface set interface \"Ethernet\" disable"])
        .output();

    if disable_result.is_err() {
        return Err("Failed to disable network interface");
    }

    let set_mac_result = Command::new("cmd")
        .args(&["/C", &format!("netsh interface set interface \"Ethernet\" static {}", new_mac)])
        .output();

    if set_mac_result.is_err() {
        return Err("Failed to set new MAC address");
    }

    let enable_result = Command::new("cmd")
        .args(&["/C", "netsh interface set interface \"Ethernet\" enable"])
        .output();

    if enable_result.is_err() {
        return Err("Failed to enable network interface");
    }

    Ok(())
}

// Generate a random MAC address
fn random_mac() -> String {
    let chars = "ABCDEF0123456789";
    let mut rng = thread_rng();

    let mac: String = (0..6).map(|_| {
        let octet: String = (0..2)
            .map(|_| chars.chars().nth(rng.gen_range(0..chars.len())).unwrap())
            .collect();
        octet
    }).collect::<Vec<_>>().join("-");

    mac
}

// Clean traces (sample of file deletion logic based on the batch script)
fn clean_traces() -> Result<(), &'static str> {
    let temp_dir = env::temp_dir();
    let paths_to_delete = vec![
        temp_dir.join("*.tmp"),
        temp_dir.join("*.mdmp"),
        std::path::PathBuf::from(r"C:\Windows\Prefetch\*.pf"),
        std::path::PathBuf::from(r"C:\Users\%username%\AppData\Local\EpicGamesLauncher\Saved\Logs\*.*"),
        std::path::PathBuf::from(r"C:\Windows\Temp\*.*"),
    ];

    for path in paths_to_delete {
        let _ = Command::new("cmd")
            .args(&["/C", &format!("del /f /s /q {}", path.to_string_lossy())])
            .output()
            .map_err(|_| "Failed to delete files")?;
    }

    Ok(())
}

// User menu handling
fn menu() {
    loop {
        println!("\n  Select an option:");
        println!("[1] Spoof Installation ID");
        println!("[2] Spoof PC Name");
        println!("[3] Spoof MAC Address");
        println!("[4] Spoof GUID");
        println!("[5] Spoof Hostname");
        println!("[6] Spoof Machine GUID");
        println!("[7] Clean Traces");
        println!("[exit] Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "1" => handle_result(spoof_installation_id(), "Installation ID spoofed successfully!"),
            "2" => handle_result(spoof_pc_name(), "PC Name spoofed successfully!"),
            "3" => handle_result(spoof_mac(), "MAC Address spoofed successfully!"),
            "4" => handle_result(spoof_guid(), "GUID spoofed successfully!"),
            "5" => handle_result(spoof_hostname(), "Hostname changed successfully!"),
            "6" => handle_result(spoof_machine_guid(), "Machine GUID spoofed successfully!"),
            "7" => handle_result(clean_traces(), "Traces cleaned successfully!"),
            "exit" => exit(0),
            _ => println!("{}", "Invalid option!".red()),
        }

        thread::sleep(Duration::from_secs(2)); // Adding delay for better UX
    }
}

fn main() {
    println!("{}", "=== HWID Spoofer ===".blue().bold());
    // Check admin privileges here
    if !is_admin() {
        println!("{}", "Attempting to relaunch with admin privileges...".yellow());
        match elevate_to_admin() {
            Ok(_) => println!("{}", "Successfully elevated to admin.".green()),
            Err(_) => {
                eprintln!("{}", "Failed to obtain admin privileges. Exiting.".red());
                exit(1);
            }
        }
    }

    println!("{}", "=== HWID Spoofer ===".blue().bold());
    menu();
}

// Check if the program is running with admin privileges
fn is_admin() -> bool {
    let output = Command::new("powershell")
        .args(&["-Command", "([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)"])
        .output()
        .expect("Failed to execute PowerShell command.");

    String::from_utf8_lossy(&output.stdout).trim() == "True"
}

// Attempt to relaunch the program with admin privileges
fn elevate_to_admin() -> Result<(), &'static str> {
    let exe_path = env::current_exe().map_err(|_| "Failed to get the executable path")?;

    Command::new("powershell")
        .args(&["-Command", &format!("Start-Process -FilePath '{}' -Verb RunAs", exe_path.display())])
        .spawn()
        .map_err(|_| "Failed to relaunch the program as admin")?;

    Ok(())
}
