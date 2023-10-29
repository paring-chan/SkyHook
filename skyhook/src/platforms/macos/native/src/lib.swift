import Cocoa

public typealias EventCallback = @convention(c) (
    UInt, Bool
) -> Void

var hook: MacOSHook? = nil

@_cdecl("start_macos_hook")
public func startMacOSHook(callback: EventCallback) -> UnsafePointer<CChar>? {
    if hook != nil {
        return ("Hook is already running" as NSString).utf8String
    }

    hook = MacOSHook(callback: callback)

    do {
        try hook!.start()
    } catch {
        hook = nil
        return (error.localizedDescription as NSString).utf8String
    }

    return nil
}

@_cdecl("stop_macos_hook")
public func stopMacOSHook(callback: EventCallback) -> UnsafePointer<CChar>? {
    if hook == nil {
        return ("Hook is not running" as NSString).utf8String
    }

    hook!.stop()

    hook = nil

    return nil
}

@_cdecl("macos_hook_running")
public func macosHookRunning() -> Bool {
    return hook != nil;
}
