<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

const isMaximized = ref(false);
const appVersion = ref("0.1.0");

async function toggleMaximize() {
  try {
    const win = getCurrentWindow();
    isMaximized.value = await win.isMaximized();
    if (isMaximized.value) {
      await win.unmaximize();
    } else {
      await win.maximize();
    }
    isMaximized.value = !isMaximized.value;
  } catch (e) {
    console.log("Window control not available in browser");
  }
}

async function minimize() {
  try {
    const win = getCurrentWindow();
    await win.minimize();
  } catch (e) {
    console.log("Window control not available in browser");
  }
}

async function close() {
  try {
    const win = getCurrentWindow();
    await win.close();
  } catch (e) {
    console.log("Window control not available in browser");
  }
}

onMounted(async () => {
  try {
    appVersion.value = await invoke<string>("get_app_version");
  } catch (e) {
    // Browser mode
  }
});
</script>

<template>
  <div class="h-screen w-screen flex flex-col bg-yueyin-bg overflow-hidden rounded-window">
    <div
      class="h-11 flex items-center justify-between px-4 bg-transparent"
      data-tauri-drag-region
    >
      <div class="flex items-center gap-2" data-tauri-drag-region>
        <span class="text-yueyin-accent font-bold text-sm tracking-wider">YUEYIN</span>
        <span class="text-yueyin-muted text-xs">v{{ appVersion }}</span>
      </div>

      <div class="flex items-center gap-1">
        <button
          @click="minimize"
          class="w-9 h-7 rounded-lg flex items-center justify-center text-yueyin-muted hover:text-yueyin-ink hover:bg-white/5 transition-colors"
        >
          <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 12H4" />
          </svg>
        </button>
        <button
          @click="toggleMaximize"
          class="w-9 h-7 rounded-lg flex items-center justify-center text-yueyin-muted hover:text-yueyin-ink hover:bg-white/5 transition-colors"
        >
          <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path
              v-if="!isMaximized"
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M4 8V4m0 0h4M4 4l5 5m11-1V4m0 0h-4m4 0l-5 5M4 16v4m0 0h4m-4 0l5-5m11 5l-5-5m5 5v-4m0 4h-4"
            />
            <path
              v-else
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3"
            />
          </svg>
        </button>
        <button
          @click="close"
          class="w-9 h-7 rounded-lg flex items-center justify-center text-yueyin-muted hover:text-white hover:bg-red-500/80 transition-colors"
        >
          <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>
    </div>

    <div class="flex-1 flex items-center justify-center">
      <div class="text-center space-y-4">
        <div class="w-24 h-24 mx-auto rounded-full bg-yueyin-accent/10 flex items-center justify-center">
          <svg
            class="w-12 h-12 text-yueyin-accent"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="1.5"
              d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3"
            />
          </svg>
        </div>
        <p class="text-yueyin-muted">播放器界面待实现</p>
        <router-link
          to="/"
          class="inline-block text-sm text-yueyin-accent hover:text-yueyin-accent-hover transition-colors"
        >
          返回首页
        </router-link>
      </div>
    </div>
  </div>
</template>
