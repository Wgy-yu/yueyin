<script setup lang="ts">
import { ref, onMounted } from "vue";

const isMaximized = ref(false);
const isTauri = ref(false);

onMounted(() => {
  // Check if running in Tauri
  isTauri.value = "__TAURI__" in window;
});

async function minimize() {
  if (!isTauri.value) {
    console.log("Window control not available in browser");
    return;
  }
  try {
    const { getCurrentWindow } = await import("@tauri-apps/api/window");
    const win = getCurrentWindow();
    await win.minimize();
  } catch (e) {
    console.error("Minimize failed:", e);
  }
}

async function toggleMaximize() {
  if (!isTauri.value) {
    console.log("Window control not available in browser");
    return;
  }
  try {
    const { getCurrentWindow } = await import("@tauri-apps/api/window");
    const win = getCurrentWindow();
    const maximized = await win.isMaximized();
    if (maximized) {
      await win.unmaximize();
    } else {
      await win.maximize();
    }
    isMaximized.value = !maximized;
  } catch (e) {
    console.error("Maximize failed:", e);
  }
}

async function close() {
  if (!isTauri.value) {
    console.log("Window control not available in browser");
    return;
  }
  try {
    const { getCurrentWindow } = await import("@tauri-apps/api/window");
    const win = getCurrentWindow();
    await win.close();
  } catch (e) {
    console.error("Close failed:", e);
  }
}
</script>

<template>
  <div id="desktop-titlebar" aria-label="window controls">
    <div class="desktop-drag-region" data-tauri-drag-region>
      <div class="desktop-app-mark" aria-hidden="true"></div>
      <div class="desktop-app-title" aria-hidden="true">悦音</div>
    </div>
    <div class="desktop-window-controls">
      <button
        class="desktop-window-btn"
        @click.stop="minimize"
        title="最小化"
        aria-label="最小化"
      >
        <svg viewBox="0 0 16 16">
          <path d="M3 8h10" />
        </svg>
      </button>
      <button
        class="desktop-window-btn"
        @click.stop="toggleMaximize"
        title="最大化"
        aria-label="最大化"
      >
        <svg v-if="!isMaximized" viewBox="0 0 16 16">
          <rect x="4" y="4" width="8" height="8" rx="1.5" />
        </svg>
        <svg v-else viewBox="0 0 16 16">
          <path d="M5 3.5h7.5v7.5" />
          <rect x="3.5" y="5.5" width="7" height="7" rx="1.3" />
        </svg>
      </button>
      <button
        class="desktop-window-btn close"
        @click.stop="close"
        title="关闭"
        aria-label="关闭"
      >
        <svg viewBox="0 0 16 16">
          <path d="M4 4l8 8M12 4l-8 8" />
        </svg>
      </button>
    </div>
  </div>
</template>

<style scoped>
#desktop-titlebar {
  position: fixed;
  z-index: 500;
  top: 0;
  left: 0;
  right: 0;
  height: 44px;
  padding: 0 12px 0 18px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  background: transparent;
  backdrop-filter: none;
  -webkit-backdrop-filter: none;
  -webkit-app-region: drag;
  pointer-events: none;
}

.desktop-drag-region {
  height: 100%;
  flex: 1;
  display: flex;
  align-items: center;
  gap: 9px;
  min-width: 0;
  color: rgba(224, 250, 255, 0.74);
  font-size: 12px;
  font-weight: 600;
  letter-spacing: 0;
  text-transform: uppercase;
  -webkit-app-region: drag;
  pointer-events: auto;
}

.desktop-app-mark {
  display: none;
}

.desktop-app-title {
  font-size: 11px;
  font-weight: 700;
  letter-spacing: 0.08em;
  color: rgba(224, 250, 255, 0.6);
}

.desktop-window-controls {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 0;
  border-radius: 0;
  background: transparent;
  box-shadow: none;
  -webkit-app-region: no-drag;
  pointer-events: auto;
}

.desktop-window-btn {
  width: 38px;
  height: 30px;
  border: 0;
  border-radius: 10px;
  background: rgba(4, 8, 10, 0.42);
  color: rgba(224, 250, 255, 0.72);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: background 0.18s, color 0.18s, transform 0.18s;
}

.desktop-window-btn svg {
  width: 14px;
  height: 14px;
  stroke: currentColor;
  stroke-width: 2;
  fill: none;
}

.desktop-window-btn:hover {
  background: rgba(244, 210, 138, 0.14);
  color: #fff1bd;
  transform: translateY(-1px);
}

.desktop-window-btn.close:hover {
  background: rgba(255, 86, 100, 0.86);
  color: #fff;
}
</style>
