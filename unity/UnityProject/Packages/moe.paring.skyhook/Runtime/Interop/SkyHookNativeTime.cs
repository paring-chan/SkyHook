using System.Runtime.InteropServices;

namespace SkyHook.Interop
{
    [StructLayout(LayoutKind.Sequential)]
    public struct SkyHookNativeTime
    {
        public long TimeSec;
        public uint TimeNSec;
    }
}