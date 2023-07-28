using System;
using SkyHook;
using UnityEngine;

namespace SkyHookTest
{
    public class SkyHookTest : MonoBehaviour
    {
        public SkyHook.SkyHook hook;

        private SkyHookEvent? _lastEvent;

        private void Start()
        {
            hook.StartHook();
        }

        private void Update()
        {
            if (Input.GetKeyDown(KeyCode.Minus))
            {
                hook.PollingFrequency -= 50;
            }

            if (Input.GetKeyDown(KeyCode.Equals))
            {
                hook.PollingFrequency += 50;
            }

            if (Input.GetKeyDown(KeyCode.Escape))
            {
                Debug.Log(hook.IsRunning);
                if (hook.IsRunning)
                {
                    hook.StopHook();
                }
                else
                {
                    hook.StartHook();
                }
            }

            if (!Application.isFocused)
            {
                hook.ReadQueue();
                return;
            }

            foreach (var ev in hook.ReadQueue())
            {
                _lastEvent = ev;
            }
        }

        private string EventToString(SkyHookEvent ev)
        {
            return $"{ev.Time} {ev.KeyCode}({ev.Key}) {ev.EventType}";
        }

        private void OnGUI()
        {
            var ev = _lastEvent.HasValue
                ? EventToString(_lastEvent.Value)
                : "None";
            GUI.Label(new Rect(0, 0, 480, 240),
                $"PollingFrequency: {hook.PollingFrequency}\nLast Event: {ev}\nRunning: {hook.IsRunning}");
        }
    }
}