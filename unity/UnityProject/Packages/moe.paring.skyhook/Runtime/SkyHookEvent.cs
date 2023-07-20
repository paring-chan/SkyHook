using System;

namespace SkyHook
{
    public struct SkyHookEvent
    {
        public SkyHookKeyCode KeyCode;
        public SkyHookEventType EventType;
        public int Key;
        public DateTime Time;
    }

    public enum SkyHookEventType
    {
        KeyPress,
        KeyRelease,
    }
}