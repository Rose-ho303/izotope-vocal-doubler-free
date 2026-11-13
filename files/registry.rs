//! Windows registry read helpers for Izotope Vocal Doubler Free.

pub const APP_KEY: &str = "Software\\Izotope\\Izotope Vocal Doubler Free";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Izotope Vocal Doubler Free";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Izotope Vocal Doubler Free") },
        RegValue { name: "Version",     value: String::from("v4.8.9") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Izotope Vocal Doubler Free") },
        RegValue { name: "Publisher",       value: String::from("Izotope") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Izotope Vocal Doubler Free\\uninstall.exe\"") },
    ]
}
