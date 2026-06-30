<script setup lang="ts">
import type { PlaylistInfo } from "../../services/music";

defineProps<{
  playlists: PlaylistInfo[];
  loggedIn: boolean;
  error?: string;
}>();
defineEmits<{ (e: "open", id: string): void }>();
</script>

<template>
  <div class="flex-1 overflow-y-auto py-2">
    <div v-if="error" class="p-10 text-center text-[13px] text-red-300/80">
      {{ error }}
    </div>
    <div v-else-if="!playlists.length" class="p-10 text-center text-[13px] text-white/40">
      {{ loggedIn ? "暂无歌单" : "请先登录" }}
    </div>
    <template v-else>
      <button
        v-for="pl in playlists"
        :key="pl.id"
        class="flex w-full items-center gap-3 px-5 py-2 text-left transition-colors hover:bg-white/[0.04]"
        @click="$emit('open', pl.id)"
      >
        <div class="h-12 w-12 shrink-0 rounded-lg bg-gradient-to-br from-[rgba(0,245,212,.2)] to-[rgba(36,66,255,.2)] bg-cover bg-center" :style="pl.cover ? { backgroundImage: `url(${pl.cover})` } : undefined"></div>
        <div class="min-w-0 flex-1">
          <div class="truncate text-[13px] font-semibold text-white/90">{{ pl.name }}</div>
          <div class="mt-0.5 text-[11px] text-white/40">{{ pl.trackCount }} 首 · {{ pl.creator }}</div>
        </div>
      </button>
    </template>
  </div>
</template>
