using System;
using System.Runtime.InteropServices;

namespace SkyHook.Interop
{
    [StructLayout(LayoutKind.Sequential)]
    public struct SkyHookNativeTime
    {
        private static readonly DateTime Epoch = new(1970, 1, 1);
 
        public long TimeSec;
        public uint TimeNSec;

        public DateTime GetDateTime()
        {
            return new DateTime(Epoch.Ticks + (TimeSec * 1000000000 + TimeNSec) / 100);
        }
    }
}