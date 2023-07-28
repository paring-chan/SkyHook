using System.Runtime.InteropServices;

namespace SkyHook.Interop
{
    internal static class SkyHookNative
    {
        private const string Lib = "skyhook_unity";

        [DllImport(Lib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "skyhook_new_hook")]
        public static extern uint NewHook();

        [DllImport(Lib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "skyhook_drop_hook")]
        public static extern void DropHook(ulong id);

        [DllImport(Lib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "skyhook_start_hook")]
        public static extern string StartHook(ulong id);

        [DllImport(Lib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "skyhook_stop_hook")]
        public static extern void StopHook(ulong id);

        public delegate void ReadQueueCallback(SkyHookNativeEvent @event);

        [DllImport(Lib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "skyhook_read_queue")]
        public static extern void ReadQueue(ulong id, ReadQueueCallback callback);

        [DllImport(Lib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "skyhook_get_time")]
        public static extern SkyHookNativeTime GetTime();

        [DllImport(Lib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "skyhook_get_polling_frequency")]
        public static extern ulong GetPollingFrequency(ulong id);

        [DllImport(Lib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "skyhook_set_polling_frequency")]
        public static extern string SetPollingFrequency(ulong id, ulong value);

        [DllImport(Lib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "skyhook_get_running")]
        public static extern bool GetRunning(ulong id);
    }
}