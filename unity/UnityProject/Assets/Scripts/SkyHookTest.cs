using UnityEngine;

namespace SkyHookTest
{
    public class SkyHookTest : MonoBehaviour
    {
        public SkyHook.SkyHook hook;
        
        private void Start()
        {
            hook.StartHook();
        }
    }
}