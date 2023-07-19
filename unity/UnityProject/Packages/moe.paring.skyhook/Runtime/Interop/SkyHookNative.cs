using System.Runtime.InteropServices;

namespace SkyHook.Interop
{
    internal static class SkyHookNative
    {
        private const string Lib = "skyhook_unity";

        [DllImport(Lib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "skyhook_new_hook")]
        public static extern uint NewHook();

        [DllImport(Lib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "skyhook_drop_hook")]
        public static extern uint DropHook(uint id);

        [DllImport(Lib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "skyhook_start_hook")]
        public static extern uint StartHook(uint id);
        
        [DllImport(Lib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "skyhook_stop_hook")]
        public static extern uint StopHook(uint id);
    }
}