# Home Assistant Tray Application

A simple systray application which has some basic utilities for controlling your Home Assistant instance.
I have also developed a similar [`gnome-extension`](https://github.com/geoph9/hass-gshell-extension) but 
the bad thing about it is that it cannot be used on non-gnome DEs. In addition, this app is built with Rust.
A difference with the gnome-shell extension is that the current extension will not show weather statistics 
(temperature and humidity) since I haven't found a way on how to do that with the `tray-item-rs` library.

The current project is built on top of [`tray-item-rs`](https://github.com/olback/tray-item-rs/blob/master/Cargo.toml)
which is a Rust library for creating tray applications (on multiple platforms). In my implementation, I will 
also try to have this app run on multiple platforms, but I cannot be sure if it will work since I mostly 
switch among Gnome, KDE, Pantheon and Budgie DEs (where you can use gtk).

## Installation

TODO

## Authentication

TODO

## Layout

TODO

## Preferences/Settings

This will take some time to be implemented.

## Security Warning

Currently, I have not used any encryption on the hass access token and it is easily accessible 
by anyone who can get access to your filesystem. My plan is to do something similar to what I did with 
the gnome shell application ([more info here](https://github.com/geoph9/hass-gshell-extension#security))
and save the token in the keyring. 
