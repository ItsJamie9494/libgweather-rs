use crate::ConditionPhenomenon;
use crate::ConditionQualifier;
use crate::FormatOptions;

use glib::translate::*;

/// A convenient way to describe the current or forecast
/// weather phenomenon, if significant, and its associated
/// modifier. If the value is not significant, the weather conditions
/// are described by gweather_info_get_sky() instead.
///
/// In general it is discouraged to use this value directly to compute
/// the forecast icon: applications should instead use
/// gweather_info_get_icon_name() or
/// gweather_info_get_symbolic_icon_name().
#[repr(C)]
pub struct Conditions {
    pub significant: bool,
    pub phenomenon: ConditionPhenomenon,
    pub qualifier: ConditionQualifier,
}

impl<'a> Conditions {
    #[doc(alias = "gweather_conditions_to_string")]
    pub fn string(&self) -> glib::GString {
        unsafe { 
            from_glib_full(ffi::gweather_conditions_to_string(self.to_glib_none().0))
        }
    }

    #[doc(alias = "gweather_conditions_to_string_full")]
    pub fn string_full(&self, options: FormatOptions) -> glib::GString {
        unsafe { 
            from_glib_full(ffi::gweather_conditions_to_string_full(self.to_glib_none().0, options.into_glib()))
        }
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *mut ffi::GWeatherConditions> for Conditions {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::GWeatherConditions, Self> {
        let ptr: *const Conditions = &*self;
        Stash(ptr as *mut ffi::GWeatherConditions, self)
    }
}
