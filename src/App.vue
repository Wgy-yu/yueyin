<script setup lang="ts">
import { onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useSettingsStore } from "./stores/settings";
import { usePlayerStore } from "./stores/player";
import { useQueueStore } from "./stores/queue";
import { useSearchStore } from "./stores/search";
import { usePlayback } from "./composables/usePlayback";

const settingsStore = useSettingsStore();
const playerStore = usePlayerStore();
const queueStore = useQueueStore();
const searchStore = useSearchStore();
usePlayback();

onMounted(async () => {
  try {
    const version = await invoke<string>("get_app_version");
    console.log("悦音版本:", version);
  } catch (e) {
    console.log("Running in browser mode");
  }
  await settingsStore.load();
  playerStore.init();
  await queueStore.load();
  await searchStore.loadHistory();
});
</script>

<template>
  <div class="h-screen w-screen bg-yueyin-bg overflow-hidden">
    <router-view v-slot="{ Component }">
      <transition name="fade" mode="out-in">
        <component :is="Component" />
      </transition>
    </router-view>
  </div>
</template>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
