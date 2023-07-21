using System;
using System.Collections.Generic;
using SkyHook.Interop;
using UnityEngine;

namespace SkyHook
{
    public class SkyHook : MonoBehaviour
    {
        private ulong _id;

        public ulong PollingFrequency
        {
            get => SkyHookNative.GetPollingFrequency(_id);
            set => SkyHookNative.SetPollingFrequency(_id, value);
        }

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
                var time = ev.Time.GetDateTime();
                
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

        #region Static

        public static DateTime Now()
        {
            var time = SkyHookNative.GetTime();
            return time.GetDateTime();
        }

        #endregion
    }
}