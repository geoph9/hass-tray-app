# Home Assistant Tray Application

A simple systray application which has some basic utilities for controlling your Home Assistant instance.
I have also developed a similar [`gnome-extension`](https://github.com/geoph9/hass-gshell-extension) but 
the bad thing about it is that it cannot be used on non-gnome DEs. In addition, this app is built with Rust.

The current project is built on top of [`tray-item-rs`](https://github.com/olback/tray-item-rs/blob/master/Cargo.toml)
which is a Rust library for creating tray applications (on multiple platforms). In my implementation, I will 
also try to have this app run on multiple platforms, but I cannot be sure if it will work since I mostly 
switch among Gnome, KDE, Pantheon and Budgie DEs (where you can use gtk).
