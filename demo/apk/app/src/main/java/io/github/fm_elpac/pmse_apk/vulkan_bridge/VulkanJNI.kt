package io.github.fm_elpac.pmse_apk.vulkan_bridge

import android.view.Surface

class VulkanJNI {
    init {
        System.loadLibrary("pmse_apk")
        // debug
        println("DEBUG: load libpmse_apk.so")
    }

    private external fun nativeInit()
    private external fun nativeCreate(surface: Surface)
    private external fun nativeDestroy()
    private external fun nativeResize(width: Int, height: Int)
    private external fun nativeDraw()

    constructor() {
        nativeInit()
    }

    fun create(surface: Surface) {
        // debug
        println("DEBUG: before nativeCreate()")

        nativeCreate(surface)
    }

    fun destroy() {
        // debug
        println("DEBUG: before nativeDestroy()")

        nativeDestroy()
    }

    fun resize(width: Int, height: Int) {
        // debug
        println("DEBUG: before nativeResize()")

        nativeResize(width, height)
    }

    fun draw() {
        // debug
        println("DEBUG: before nativeDraw()")

        nativeDraw()
    }
}
