package io.github.fm_elpac.pmse_apk

import android.os.Bundle
import android.app.Activity

import io.github.fm_elpac.pmse_apk.vulkan_bridge.VulkanSurfaceView

class MainActivity : Activity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        // TODO

        var v = VulkanSurfaceView(this)
        setContentView(v)
    }
}
