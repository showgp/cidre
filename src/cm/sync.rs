use crate::{cf, cm, define_cf_type};

define_cf_type!(Clock(cf::Type));
define_cf_type!(Timebase(cf::Type));
define_cf_type!(ClockOrTimebase(cf::Type));

impl Clock {
    pub fn get_type_id() -> cf::TypeId {
        unsafe { CMClockGetTypeID() }
    }

    /// Returns a reference to the singleton clock logically identified with host time.
    #[inline]
    pub fn host_time_clock<'a>() -> &'a Clock {
        unsafe { &CMClockGetHostTimeClock() }
    }

    #[inline]
    pub fn time(&self) -> cm::Time {
        unsafe { CMClockGetTime(self) }
    }

    /// Converts a host time from CMTime to the host time's native units.
    /// 
    /// This function performs a scale conversion, not a clock conversion.
		/// It can be more accurate than CMTimeConvertScale because the system units may 
		/// have a non-integer timescale.
		/// On Mac OS X, this function converts to the units of mach_absolute_time.
    #[inline]
    pub fn convert_host_time_to_system_units(host_time: cm::Time) -> u64 {
        unsafe { CMClockConvertHostTimeToSystemUnits(host_time) }
    }

    /// Converts a host time from native units to cm::Time.
    #[inline]
    pub fn make_host_time_from_system_units(host_time: u64) -> cm::Time {
        unsafe { CMClockMakeHostTimeFromSystemUnits(host_time) }
    }
}

extern "C" {
    fn CMClockGetTypeID() -> cf::TypeId;
    fn CMClockGetHostTimeClock<'a>() -> &'a Clock;

    fn CMClockGetTime(clock: &Clock) -> cm::Time;
    fn CMClockConvertHostTimeToSystemUnits(host_time: cm::Time) -> u64;
    fn CMClockMakeHostTimeFromSystemUnits(host_time: u64) -> cm::Time;
}