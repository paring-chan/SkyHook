import Foundation
import Cocoa

class MacOSHook {
    var callback: EventCallback
    let thread: MacOSHookThread

    init(callback: EventCallback) {
        self.callback = callback

        self.thread = MacOSHookThread()

        self.thread.setup(hook: self)
    }

    func start() throws {
        thread.start()

        while (true) {
            if (thread.started || thread.error != nil) {
                break
            }
        }

        if (thread.error != nil) {
            throw thread.error!
        }
    }

    func stop() {
        if (self.thread.runLoop != nil) {
            CFRunLoopStop(self.thread.runLoop)
        }

        while (true) {
            if (!self.thread.started) {
                break
            }
        }
    }
}
