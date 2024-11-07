use swift_rs::{swift, Int, Float};

/// A safe wrapper around an unsafe call to swift UIKit
/// to get the current battery level for an iOS device.
/// 
/// Returns an enum with the following values:
/// - `Unknown`: maps to the UIKit unknown status
/// - `Discharging(f32)`: Not plugged in, the f32 value
///     represents the charge level of the battery (0-1)
/// - `Charging(f32)`: Plugged in, the f32 value
///     represents the level of the battery (0-1)
/// - `Error`: The function encoutered an unexpected
///     value and the state is unreliable
/// ```
/// use swift_bat::get_battery_state;
/// use swift_bat::BatteryState;
/// use swift_bat::Error;
/// 
/// let battery = unsafe { get_battery_state() };
/// assert_ne!(battery, BatteryState::Error)
/// ```
/// 
/// BatteryState implements serde and is represented
/// as an internally tagged object, with state being
/// a string of the enum tag, and content being the
/// battery level.
pub async fn get_battery_state() -> crate::BatteryState {
    let battery = unsafe { get_swift_battery() };
    return (&*battery).into()
}

/// this function FFIs to swift. Must always be
/// wrapped in an unsafe block
swift!(
    fn get_swift_battery() -> swift_rs::SRObject<SwiftBattery>
);

/// representation of the object returned through
/// Objective-C from Swift
#[repr(C)]
pub(crate) struct SwiftBattery {
    level: Float,
    state: Int
}

/// Rust version of the battery state.
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag="status", content="level")]
pub enum BatteryState {
    Charging(f32),
    Discharging(f32),
    Unknown,
    Error
}

impl From<&SwiftBattery> for BatteryState {
    fn from(bat: &SwiftBattery) -> Self {
        match bat.state {
            0 => BatteryState::Unknown,
            1 => BatteryState::Discharging(bat.level),
            2 => BatteryState::Charging(bat.level),
            3 => BatteryState::Charging(1.0),
            _ => BatteryState::Error
        }
    }
}
impl BatteryState {
    /// Returns true if the battery meets the criteria of Apple's
    /// full state. That is: plugged in and 100%
    pub fn is_full(&self) -> bool {
        match self {
            Self::Charging(val) if *val == 100.0 => true,
            _ => false,
        }
    }
    /// Returns the level for the battery. Like in the Swift API,
    /// if the battery is in error or unknown status, it returns
    /// -1
    pub fn level(&self) -> f32 {
        match self {
            Self::Charging(val) | Self::Discharging(val) => *val,
            _ => -1.0,
        }
    }
}
#[derive(Debug)]
pub enum Error {
    BatteryMissing,
}

#[cfg(test)]
mod tests {
    use crate::BatteryState;

    swift_rs::swift!(fn get_swift_battery_level_test() -> swift_rs::SRObject<super::SwiftBattery>);

    #[test]
    /// asserts that the battery state retuned maps correctly to
    /// the BatteryState enum.
    pub fn get_swift_battery_returns_state() {
        let bat = unsafe { get_swift_battery_level_test() };
        assert_eq!(bat.state, 2);
    }
    #[test]
    pub fn test_that_battery_level_returns_level() {
        let bat = unsafe { get_swift_battery_level_test() };
        assert_eq!(bat.level, 0.76);
    }
    #[test]
    pub fn test_that_battery_status_calls_swift() {
        let battery: super::BatteryState = unsafe { (&*get_swift_battery_level_test()).into() };
        assert_eq!(battery, BatteryState::Charging(0.76));
    }
}