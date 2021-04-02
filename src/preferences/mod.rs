#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
mod macos;

// Set type depending on OS
#[cfg(target_os = "linux")]
pub type Preferences = linux::PreferencesLinux;

#[cfg(target_os = "windows")]
pub type Preferences = windows::PreferencesWindows;

#[cfg(target_os = "macos")]
pub type Preferences = macos::PreferencesMacOS;