import Foundation
import UIKit

public class Battery: NSObject {
	var level: Float
	var state: Int

	init(_ level: Float, _ state: Int) {
		self.level = level;
		self.state = state;
	}
}


@MainActor
@_cdecl("get_swift_battery")
public func getSwiftBattery() -> Battery {
	UIDevice.current.isBatteryMonitoringEnabled = true;
	var batteryLevel: Float { UIDevice.current.batteryLevel }
    var batteryState: UIDevice.BatteryState { UIDevice.current.batteryState }

    return Battery(batteryLevel, getBatteryStateValue(state:batteryState))
}

@_cdecl("get_swift_battery_level_test")
public func getSwiftBatteryLevelTest() -> Battery {
	return Battery(0.76, 2)
}

 func getBatteryStateValue(state: UIDevice.BatteryState) -> Int {
     switch state {
         case .full: return 3
         case .unplugged: return 1
         case .charging: return 2
         case .unknown: return 0
     default: return 4
     }
 }
