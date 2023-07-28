using System;
using System.Collections.Generic;
using System.Threading;
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
            set
            {
                var result = SkyHookNative.SetPollingFrequency(_id, value);
                if (!string.IsNullOrEmpty(result)) throw new SkyHookException(result);
            }
        }

        public bool IsRunning => SkyHookNative.GetRunning(_id);

        private Thread _thread;
        private bool _dummy;

        private void Awake()
        {
            _id = SkyHookNative.NewHook();
        }

        public void StartHook()
        {
            var started = false;
            string error = null;
            
            _thread = new Thread(() =>
            {
                var result = SkyHookNative.StartHook(_id);
                if (!string.IsNullOrEmpty(result))
                {
                    error = result;
                    return;
                }

                started = true;

                while (IsRunning)
                {
                    _dummy = !_dummy;
                }
            });
            
            _thread.Start();

            while (!started)
            {
                if (!string.IsNullOrEmpty(error))
                {
                    throw new SkyHookException(error);
                }
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