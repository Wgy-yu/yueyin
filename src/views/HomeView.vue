<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke, isTauri } from "@tauri-apps/api/core";
import SplashScreen from "../components/SplashScreen.vue";
import WindowControls from "../components/WindowControls.vue";
import SearchArea from "../components/SearchArea.vue";
import TopRight from "../components/TopRight.vue";
import HomeContent from "../components/HomeContent.vue";
import BottomBar from "../components/BottomBar.vue";
import LyricsPanel from "../components/LyricsPanel.vue";

const showSplash = ref(true);
const appVersion = ref("0.1.0");
const isMaximized = ref(false);
const isFullscreen = ref(false);

function handleSplashEnter() {
  showSplash.value = false;
}

onMounted(async () => {
  try {
    appVersion.value = await invoke<string>("get_app_version");
  } catch (e) {
    // Browser mode
  }

  // Listen for window state changes
  if (isTauri()) {
    try {
      const { getCurrentWindow } = await import("@tauri-apps/api/window");
      const win = getCurrentWindow();
      isMaximized.value = await win.isMaximized();

      win.onResized(() => {
        win.isMaximized().then((maximized) => {
          isMaximized.value = maximized;
        });
      });
    } catch (e) {
      console.log("Window state listener not available");
    }
  }
});
</script>

<template>
  <div
    id="app-window"
    class="h-screen w-screen overflow-hidden bg-[#010304] relative"
    :class="{
      maximized: isMaximized,
      fullscreen: isFullscreen,
    }"
  >
    <!-- Background -->
    <div id="custom-bg">
      <div class="bg-gradient"></div>
    </div>

    <!-- Canvas container for visuals -->
    <div id="canvas-container"></div>

    <!-- Splash Screen -->
    <SplashScreen v-if="showSplash" @enter="handleSplashEnter" />

    <!-- Window Controls -->
    <WindowControls />

    <!-- Search Area -->
    <SearchArea />

    <!-- Top Right -->
    <TopRight />

    <!-- Home Content -->
    <HomeContent />

    <!-- Lyrics Panel -->
    <LyricsPanel />

    <!-- Bottom Bar -->
    <BottomBar />
  </div>
</template>

<style scoped>
#app-window {
  border-radius: 34px;
  overflow: hidden;
  box-shadow: 0 24px 80px rgba(0, 0, 0, 0.46);
  transition: border-radius 0.2s ease;
  clip-path: inset(0 round 34px);
  transform: translateZ(0);
}

#app-window.maximized,
#app-window.fullscreen {
  border-radius: 0;
  box-shadow: none;
  clip-path: none;
}

#custom-bg {
  position: absolute;
  inset: 0;
  z-index: 0;
  background: #000;
  overflow: hidden;
  transition: background 0.35s ease;
}

.bg-gradient {
  position: absolute;
  inset: 0;
  background: linear-gradient(
    180deg,
    rgba(0, 0, 0, 0.06),
    rgba(0, 0, 0, 0.16)
  );
}

#canvas-container {
  position: absolute;
  inset: 0;
  z-index: 1;
  transition: opacity 1450ms cubic-bezier(0.16, 1, 0.3, 1),
    transform 1450ms cubic-bezier(0.16, 1, 0.3, 1);
}
</style>
