using System.Runtime.InteropServices;

namespace SkyHook.Interop
{
    [StructLayout(LayoutKind.Sequential)]
    internal struct SkyHookNativeEvent
    {
        public SkyHookKeyCode KeyCode;
        public SkyHookEventType EventType;
        public int Key;
        public ulong TimeSec;
        public ulong TimeNSec;
    }
}