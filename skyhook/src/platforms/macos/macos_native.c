#include <Cocoa/Cocoa.h>
#include <pthread.h>
#include <stdio.h>

uint16_t pressed_keys = 0;

bool is_pressed(uint16_t key) { return pressed_keys & key; }

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
    key = 0;
    down = true;
    break;
  case kCGEventLeftMouseUp:
    key = 0;
    break;
  case kCGEventRightMouseDown:
    key = 1;
    down = true;
    break;
  case kCGEventRightMouseUp:
    key = 1;
    break;
  case kCGEventOtherMouseDown:
    key = 2;
    down = true;
    break;
  case kCGEventOtherMouseUp:
    key = 2;
    break;
  case kCGEventKeyDown:
    down = true;
    key = (uint)CGEventGetIntegerValueField(event, kCGKeyboardEventKeycode) + 3;
    break;
  case kCGEventKeyUp:
    key = (uint)CGEventGetIntegerValueField(event, kCGKeyboardEventKeycode) + 3;
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

  printf("%d %d\n", key, down);

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

  eventTap = CGEventTapCreate(kCGSessionEventTap, kCGHeadInsertEventTap, 0,
                              eventMask, tap_callback, NULL);

  if (!eventTap) {
    error_content = "failed to create event tap";
    thread_id = NULL;
    started = false;
    return NULL;
  }

  runLoopSource =
      CFMachPortCreateRunLoopSource(kCFAllocatorDefault, eventTap, 0);

  loop = CFRunLoopGetCurrent();

  CFRunLoopAddSource(loop, runLoopSource, kCFRunLoopCommonModes);

  CGEventTapEnable(eventTap, true);

  started = true;

  CFRunLoopRun();

  started = false;

  thread_id = NULL;

  loop = NULL;

  return NULL;
}

char *start_macos_hook() {
  if (thread_id != NULL || started) {
    return "Hook is already started";
  }

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
