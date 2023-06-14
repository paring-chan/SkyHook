import Cocoa

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

        let monitor = NSEvent.addLocalMonitorForEvents(matching: [
            NSEvent.EventTypeMask.leftMouseDown,
            NSEvent.EventTypeMask.leftMouseUp,
            NSEvent.EventTypeMask.rightMouseDown,
            NSEvent.EventTypeMask.rightMouseUp,
            NSEvent.EventTypeMask.otherMouseDown,
            NSEvent.EventTypeMask.otherMouseUp,
            NSEvent.EventTypeMask.keyDown,
            NSEvent.EventTypeMask.keyUp,
        ], handler: eventCallback)

        if (monitor == nil) {
            throw HookError.MonitorCreateFailed
        }

        runLoop = CFRunLoopGetCurrent()

        self.started = true

        CFRunLoopRun()

        NSEvent.removeMonitor(monitor!)

        runLoopSource = nil

        self.started = false
    }

    func eventCallback(event: NSEvent) -> NSEvent {
        print(event)

        return event
    }
}