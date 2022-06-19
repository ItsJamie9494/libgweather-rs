#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![allow(clippy::needless_doctest_main)]
#![doc(
    html_logo_url = "https://gitlab.gnome.org/GNOME/libgweather/-/raw/main/doc/libgweather-logo.svg",
    html_favicon_url = "https://gitlab.gnome.org/GNOME/libgweather/-/raw/main/doc/libgweather-logo.svg"
)]
//! # Rust GWeather bindings
//!
//! This library contains safe Rust bindings for [GWeather](https://gitlab.gnome.org/GNOME/libgweather/), a library that offers
//! weather data collection
//!
//! See also
//!
//! - [GWeather documentation](https://gnome.pages.gitlab.gnome.org/libgweather/)
//! - [gtk-rs project overview](https://gtk-rs.org/)
//!



macro_rules! skip_assert_initialized {
    () => {};
}

macro_rules! assert_initialized_main_thread {
    () => {}; // if !::gtk::is_initialized_main_thread() {
              //     if ::gtk::is_initialized() {
              //         panic!("libgweather may only be used from the main thread.");
              //     } else {
              //         panic!("Gtk has to be initialized before using libgweather.");
              //     }
              // }
}

pub use ffi;
pub use gio;
pub use glib;

#[allow(unused_imports)]
#[allow(clippy::let_and_return)]
#[allow(clippy::type_complexity)]
mod auto;

mod location;
mod info;
mod conditions;

pub use auto::*;
