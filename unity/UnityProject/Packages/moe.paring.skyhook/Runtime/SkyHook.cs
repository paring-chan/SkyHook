using System;
using System.Collections.Generic;
using SkyHook.Interop;
using UnityEngine;

namespace SkyHook
{
    public class SkyHook : MonoBehaviour
    {
        private static readonly long EpochTicks = new DateTime(1970,  1, 1).Ticks;
        
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
            SkyHookNative.ReadQueue(_id, @event =>
            {
                var time = new DateTime(EpochTicks + (long)(@event.TimeSec * 10000000L) + (long)(@event.TimeNSec / 100L));

                result.Add(new SkyHookEvent
                {
                    KeyCode = @event.KeyCode,
                    EventType = @event.EventType,
                    Key = @event.Key,
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