/* THIS FILE IS AUTO-GENERATED. DO NOT MODIFY!! */

// Copyright 2020-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

package com.tris.tasktimer.app

import android.annotation.SuppressLint
import android.webkit.*
import android.content.Context
import android.os.Build
import kotlin.collections.Map
import android.view.View

class RustWebView(context: Context): WebView(context) {
    init {
        settings.javaScriptEnabled = true
        settings.domStorageEnabled = true
        settings.setGeolocationEnabled(true)
        settings.databaseEnabled = true
        settings.mediaPlaybackRequiresUserGesture = false
        settings.javaScriptCanOpenWindowsAutomatically = true
        this.overScrollMode = View.OVER_SCROLL_NEVER
        this.setVerticalScrollBarEnabled(false);
        
    }

    fun loadUrlMainThread(url: String) {
        post {
          super.loadUrl(url)
        }
    }

    fun loadUrlMainThread(url: String, additionalHttpHeaders: Map<String, String>) {
        post {
          super.loadUrl(url, additionalHttpHeaders)
        }
    }

    
}
