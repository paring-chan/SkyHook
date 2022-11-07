#include <Cocoa/Cocoa.h>
#include <pthread.h>
#include <stdio.h>

typedef void (*event_callback)(uint, bool);

event_callback current_callback;

void process_flags_changed(CGEventRef event, uint *key, bool *down,
                           bool *exists) {
  uint code = CGEventGetIntegerValueField(event, kCGKeyboardEventKeycode);
  CGEventFlags mask = CGEventGetFlags(event);

  *key = code;

  switch (code) {
  case 59: // left control
    *exists = true;
    if (mask & kCGEventFlagMaskControl) {
      *down = true;
    }
    break;
  case 62: // right control
    *exists = true;
    if (mask & kCGEventFlagMaskControl) {
      *down = true;
    }
    break;
  case 56: // left shift
    *exists = true;
    if (mask & kCGEventFlagMaskShift) {
      *down = true;
    }
    break;
  case 55: // command
    *exists = true;
    if (mask & kCGEventFlagMaskCommand) {
      *down = true;
    }
    break;
  case 60: // right shift
    *exists = true;
    if (mask & kCGEventFlagMaskShift) {
      *down = true;
    }
    break;
  case 255:
  case 57:
    return;
  default:
    printf("Unknown key: %d\n", code);
    *exists = false;
    return;
  }
}

CGEventRef tap_callback(__attribute__((unused)) CGEventTapProxy proxy,
                        CGEventType type, CGEventRef event,
                        __attribute__((unused)) void *refcon) {
  uint key;
  bool down = false;
  bool exists;

  switch (type) {
  case kCGEventLeftMouseDown:
    key = 0x100;
    down = true;
    break;
  case kCGEventLeftMouseUp:
    key = 0x100;
    break;
  case kCGEventRightMouseDown:
    key = 0x101;
    down = true;
    break;
  case kCGEventRightMouseUp:
    key = 0x101;
    break;
  case kCGEventOtherMouseDown:
    key = 0x102;
    down = true;
    break;
  case kCGEventOtherMouseUp:
    key = 0x102;
    break;
  case kCGEventKeyDown:
    down = true;
    key = (uint)CGEventGetIntegerValueField(event, kCGKeyboardEventKeycode);
    break;
  case kCGEventKeyUp:
    key = (uint)CGEventGetIntegerValueField(event, kCGKeyboardEventKeycode);
    break;
  case kCGEventFlagsChanged:
    process_flags_changed(event, &key, &down, &exists);
    if (!exists) {
      return event;
    }
    break;
  default:
    return event;
  }

  if (current_callback != NULL) {
    current_callback(key, down);
  }

  return event;
}

CGEventMask eventMask =
    ((1 << kCGEventKeyDown) | (1 << kCGEventKeyUp) |
     (1 << kCGEventLeftMouseDown) | (1 << kCGEventLeftMouseUp) |
     (1 << kCGEventRightMouseDown) | (1 << kCGEventRightMouseUp) |
     (1 << kCGEventOtherMouseDown) | (1 << kCGEventOtherMouseUp) |
     (1 << kCGEventFlagsChanged));

CFRunLoopRef loop;
bool started;
pthread_t thread_id = NULL;
char *error_content = NULL;

void *runHook(__attribute__((unused)) void *vargp) {
  if (started) {
    thread_id = NULL;
    error_content = "Hook is already started";
    return NULL;
  }

  CFMachPortRef eventTap;
  CFRunLoopSourceRef runLoopSource;

  eventTap =
      CGEventTapCreate(kCGSessionEventTap, kCGHeadInsertEventTap,
                       kCGEventTapOptionDefault, eventMask, tap_callback, NULL);

  if (!eventTap) {
    error_content = "failed to create event tap";
    thread_id = NULL;
    started = false;
    return NULL;
  }

  runLoopSource =
      CFMachPortCreateRunLoopSource(kCFAllocatorDefault, eventTap, 0);

  loop = CFRunLoopGetCurrent();

  CFRunLoopAddSource(loop, runLoopSource, kCFRunLoopDefaultMode);

  CGEventTapEnable(eventTap, true);

  started = true;

  CFRunLoopRun();

  if (CFRunLoopContainsSource(loop, runLoopSource, kCFRunLoopDefaultMode)) {
    CFRunLoopRemoveSource(loop, runLoopSource, kCFRunLoopDefaultMode);
  }

  CGEventTapEnable(eventTap, false);

  CFRelease(runLoopSource);

  CFMachPortInvalidate(eventTap);
  CFRelease(eventTap);

  eventTap = NULL;

  runLoopSource = NULL;

  thread_id = NULL;

  loop = NULL;

  started = false;

  return NULL;
}

char *start_macos_hook(event_callback callback) {
  if (thread_id != NULL || started) {
    return "Hook is already started";
  }

  current_callback = callback;

  pthread_create(&thread_id, NULL, runHook, NULL);

  while (!started && error_content == NULL) {
  }

  if (error_content != NULL) {
    return error_content;
  }

  return NULL;
}

char *stop_macos_hook() {
  if (thread_id == NULL && !started) {
    return "Hook is not started yet";
  }

  if (loop == NULL) {
    return "Failed to get run loop ref";
  }

  CFRunLoopStop(loop);

  while (thread_id != NULL || started || loop != NULL) {
  }

  return NULL;
}