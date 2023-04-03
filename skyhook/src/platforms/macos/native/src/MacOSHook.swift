import Foundation
import Cocoa

class MacOSHook {
    var callback: EventCallback
    let thread: MacOSHookThread
    private var dummyCounter = 0

    init(callback: EventCallback) {
        self.callback = callback

        self.thread = MacOSHookThread()

        self.thread.setup(hook: self)
    }

    func start() throws {
        thread.start()

        while (!thread.started && thread.error == nil) {
            dummyCounter += 1
        }

        if (thread.error != nil) {
            throw thread.error!
        }
    }

    func stop() {
        if (self.thread.runLoop != nil) {
            CFRunLoopStop(self.thread.runLoop)
        }

        while (self.thread.started) {
            dummyCounter += 1
        }
    }
}
