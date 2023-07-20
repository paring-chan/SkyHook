using System;
using SkyHook.Interop;
using UnityEngine;

namespace SkyHook
{
    public class SkyHook : MonoBehaviour
    {
        private uint _id;
        
        private void Awake()
        {
            _id = SkyHookNative.NewHook();
        }

        public void StartHook()
        {
            SkyHookNative.StartHook(_id);
        }

        public void StopHook()
        {
            SkyHookNative.StopHook(_id);
        }

        private void OnDestroy()
        {
            SkyHookNative.StopHook(_id);
            SkyHookNative.DropHook(_id);
        }
    }
}