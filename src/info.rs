use crate::Info;

use glib::translate::*;

impl Info {
    #[doc(alias = "gweather_info_get_update")]
    pub fn get_update(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gweather_info_get_update(self.to_glib_none().0))
        }
    }

    /// Requests a reload of weather conditions and forecast data from
    /// enabled network services.
    /// This call does no synchronous IO: rather, the result is delivered
    /// by emitting the #GWeatherInfo::updated signal.
    /// Note that if no network services are enabled, the signal will not
    /// be emitted. See #GWeatherInfo:enabled-providers for details.
    #[doc(alias = "gweather_info_update")]
    pub fn update(&self) {
        unsafe {
            ffi::gweather_info_update(self.to_glib_none().0);
        }
    }
}