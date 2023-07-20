using System;
using System.Collections.Generic;
using SkyHook.Interop;
using UnityEngine;

namespace SkyHook
{
    public class SkyHook : MonoBehaviour
    {
        private static readonly DateTime Epoch = new(1970, 1, 1);

        private uint _id;

        private void Awake()
        {
            _id = SkyHookNative.NewHook();
        }

        public void StartHook()
        {
            var result = SkyHookNative.StartHook(_id);
            if (!string.IsNullOrEmpty(result))
            {
                throw new SkyHookException(result);
            }
        }

        public SkyHookEvent[] ReadQueue()
        {
            var result = new List<SkyHookEvent>();
            SkyHookNative.ReadQueue(_id, ev =>
            {
                var time = new DateTime(Epoch.Ticks + (ev.TimeSec * 1000000000 + (long)ev.TimeNSec) / 100);
                
                result.Add(new SkyHookEvent
                {
                    KeyCode = ev.KeyCode,
                    EventType = ev.EventType,
                    Key = ev.Key,
                    Time = time
                });
            });

            return result.ToArray();
        }

        public void StopHook()
        {
            SkyHookNative.StopHook(_id);
        }

        private void OnDestroy()
        {
            StopHook();
            SkyHookNative.DropHook(_id);
        }
    }
}