import Cocoa

// TODO allow multi instance
private func eventCallbackWrapper(proxy: OpaquePointer, type: CGEventType, event: CGEvent, refcon: UnsafeMutableRawPointer?) -> Unmanaged<CGEvent>? {
    return hook?.thread.eventCallback(proxy: proxy, type: type, event: event, refcon: refcon)
}

class MacOSHookThread : Thread {
    var hook: MacOSHook? = nil
    var runLoopSource: CFRunLoopSource? = nil
    var runLoop: CFRunLoop? = nil
    var error: Error? = nil
    var started: Bool = false

    func setup(hook: MacOSHook) {
        self.hook = hook
    }

    override func main() {
        do {
            try startHook()
        } catch {
            self.error = error
        }
    }

    private var pressedCache = Set<UInt>()

    private func flagsChanged(event: CGEvent, key: inout UInt, down: inout Bool) {
        let code = (event.getIntegerValueField(CGEventField.keyboardEventKeycode) as NSNumber).uintValue

        key = code

        if (pressedCache.contains(code)) {
            pressedCache.remove(code)
            down = false
        } else {
            pressedCache.insert(code)
            down = true
        }
    }

    private func startHook() throws -> Void {
        var eventMask = (1 << CGEventType.keyDown.rawValue)
        eventMask |= (1 << CGEventType.keyUp.rawValue)
        eventMask |= (1 << CGEventType.leftMouseDown.rawValue)
        eventMask |= (1 << CGEventType.leftMouseUp.rawValue)
        eventMask |= (1 << CGEventType.rightMouseDown.rawValue)
        eventMask |= (1 << CGEventType.rightMouseUp.rawValue)
        eventMask |= (1 << CGEventType.otherMouseDown.rawValue)
        eventMask |= (1 << CGEventType.otherMouseUp.rawValue)
        eventMask |= (1 << CGEventType.flagsChanged.rawValue)
                

        var eventTap: CFMachPort? = CGEvent.tapCreate(
                tap: CGEventTapLocation.cgSessionEventTap,
                place: CGEventTapPlacement.headInsertEventTap,
                options: CGEventTapOptions.defaultTap,
                eventsOfInterest: CGEventMask(eventMask),
                callback: eventCallbackWrapper,
                userInfo: nil
            )
        
        if (eventTap == nil) {
            throw HookError.EventTapCreateFailed
        }
        
        runLoopSource = CFMachPortCreateRunLoopSource(kCFAllocatorDefault, eventTap, 0)

        runLoop = CFRunLoopGetCurrent()

        CFRunLoopAddSource(runLoop, runLoopSource, CFRunLoopMode.defaultMode)

        CGEvent.tapEnable(tap: eventTap!, enable: true)

        self.started = true

        CFRunLoopRun()

        runLoopSource = nil

        CFMachPortInvalidate(eventTap!)

        eventTap = nil

        self.started = false
    }

    func eventCallback(proxy: OpaquePointer, type: CGEventType, event: CGEvent, refcon: UnsafeMutableRawPointer?) -> Unmanaged<CGEvent>? {
        var key: UInt = 0
        var isDown: Bool = false

        switch (type) {
            case .keyDown:
                key = (event.getIntegerValueField(CGEventField.keyboardEventKeycode) as NSNumber).uintValue
                isDown = true
                break
            case .keyUp:
                key = (event.getIntegerValueField(CGEventField.keyboardEventKeycode) as NSNumber).uintValue
                isDown = false
                break
            case .leftMouseDown:
                isDown = true
                key = 0x100
                break
            case .leftMouseUp:
                key = 0x100
                isDown = false
                break
            case .rightMouseDown:
                key = 0x101
                isDown = true
                break
            case .rightMouseUp:
                key = 0x101
                isDown = false
                break
            case .otherMouseDown:
                key = 0x102
                isDown = true
                break
            case .otherMouseUp:
                key = 0x102
                isDown = false
                break
            case .flagsChanged:
                flagsChanged(event: event, key: &key, down: &isDown)
                break
            default:
                return Unmanaged.passRetained(event)
        }

        self.hook?.callback(key, isDown)
        
        return Unmanaged.passRetained(event)
    }
}