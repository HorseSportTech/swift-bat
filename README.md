# Swift Bat

Swift Bat is a rust crate which gets battery info for iOS devices using Swift bindings.

## API

Swift Bat has only a simple API, the public function get_battery_state returns an enum representing the battery state. If the battery is charging or charged, it returns the value of the battery in 0.0-1.0 f32.

*IMPORTANT* Swift Bat does not include Apple's Full value. Instead, the level is 100.% and the State is Charging when the full criteria are met. If you want the Apple full definition, use the `is_full()` method.

## Useage

This module only works for iOS due to it relying on UIKit. It is recommended to place the dependancy in a conditional dependency if you have any other targets.
```[target.'cfg(target_os = "ios")'.dependencies]```