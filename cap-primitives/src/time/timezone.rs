use ambient_authority::AmbientAuthority;
use iana_time_zone::get_timezone;

/// A reference to a timezone resource.
pub struct Timezone(());

#[derive(Debug)]
pub struct TimezoneError(String);

//impl LocalResult for TimezoneError {
impl Timezone {
    /// Constructs a new instance of `Self`.
    ///
    /// # Ambient Authority
    ///
    /// This uses ambient authority to accesses clocks.
    #[inline]
    pub const fn new(ambient_authority: AmbientAuthority) -> Self {
        let _ = ambient_authority;
        Self(())
    }

    /// Returns the combined date and time with timezone.
    ///
    /// Converts NaiveTime to DateTime
    #[inline]
    pub fn timezone_name(&self) -> Result<String, TimezoneError> {
        get_timezone().map_err(|e| TimezoneError(e.to_string()))
    }
}
