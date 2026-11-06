package dev.daywright.app

import android.app.Activity
import android.os.Bundle
import android.widget.TextView

/**
 * Placeholder entry point. The real app (M2) is the phone gateway: pair any
 * Bluetooth earpiece, open a streaming session to the backend, play guidance to
 * the ear, and show a minimal day log. See docs/roadmap.md.
 *
 * Kept as a plain [Activity] with no UI framework dependency so the project
 * opens and builds with the smallest possible surface. This has not been built.
 */
class MainActivity : Activity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val view = TextView(this).apply {
            text = getString(R.string.placeholder)
            textSize = 18f
            setPadding(48, 48, 48, 48)
        }
        setContentView(view)
    }
}
