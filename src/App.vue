<script setup lang="ts">
import { onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

onMounted(async () => {
  try {
    const version = await invoke<string>("get_app_version");
    console.log("悦音版本:", version);
  } catch (e) {
    console.log("Running in browser mode");
  }
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
