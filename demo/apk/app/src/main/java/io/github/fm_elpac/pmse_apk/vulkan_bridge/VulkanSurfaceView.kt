package io.github.fm_elpac.pmse_apk.vulkan_bridge

import android.content.Context
import android.util.AttributeSet
import android.view.Surface
import android.view.SurfaceHolder
import android.view.SurfaceView

class VulkanSurfaceView: SurfaceView, SurfaceHolder.Callback2 {
    private var b = VulkanJNI()

    // constructor just call super
    constructor(context: Context): super(context) {
    }
    constructor(context: Context, attrs: AttributeSet): super(context, attrs) {
    }
    constructor(context: Context, attrs: AttributeSet, defStyle: Int): super(context, attrs, defStyle) {
    }
    constructor(context: Context, attrs: AttributeSet, defStyle: Int, defStyleRes: Int): super(context, attrs, defStyle, defStyleRes) {
    }

    init {
        alpha = 1F
        holder.addCallback(this)
    }

    // TODO GLSurfaceView

    override fun surfaceChanged(holder: SurfaceHolder, format: Int, width: Int, height: Int) {
        b.resize(width, height)
    }

    override fun surfaceDestroyed(holder: SurfaceHolder) {
        b.destroy()
    }

    override fun surfaceCreated(holder: SurfaceHolder) {
        holder.let { h ->
            b.create(h.surface)
        }
    }

    override fun surfaceRedrawNeeded(holder: SurfaceHolder) {
        b.draw()
    }
}
