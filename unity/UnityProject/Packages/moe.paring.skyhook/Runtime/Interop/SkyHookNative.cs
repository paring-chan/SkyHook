using System.Runtime.InteropServices;

namespace SkyHook.Interop
{
    internal static class SkyHookNative
    {
        private const string Lib = "skyhook_unity";

        [DllImport(Lib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "skyhook_new_hook")]
        public static extern uint NewHook();

        [DllImport(Lib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "skyhook_drop_hook")]
        public static extern void DropHook(uint id);

        [DllImport(Lib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "skyhook_start_hook")]
        public static extern string StartHook(uint id);

        [DllImport(Lib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "skyhook_stop_hook")]
        public static extern void StopHook(uint id);

        public delegate void ReadQueueCallback(SkyHookNativeEvent @event);

        [DllImport(Lib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "skyhook_read_queue")]
        public static extern void ReadQueue(uint id, ReadQueueCallback callback);
        
        [DllImport(Lib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "skyhook_get_time")]
        public static extern SkyHookNativeTime GetTime();
    }
}