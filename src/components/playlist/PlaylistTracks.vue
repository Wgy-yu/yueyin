<script setup lang="ts">
import type { Track } from "../../types/track";

defineProps<{ tracks: Track[]; error?: string }>();
defineEmits<{ (e: "play-all"): void; (e: "play-track", index: number): void }>();
</script>

<template>
  <div class="flex flex-1 flex-col overflow-hidden">
    <div class="flex items-center justify-between px-5 py-2.5 text-[12px] text-white/50">
      <span>{{ tracks.length }} 首歌曲</span>
      <button class="rounded-lg border border-[rgba(0,245,212,.25)] bg-[rgba(0,245,212,.08)] px-3.5 py-1 text-[11px] font-semibold text-[rgba(0,245,212,.9)] transition-colors hover:bg-[rgba(0,245,212,.16)]" @click="$emit('play-all')">播放全部</button>
    </div>
    <div v-if="error" class="p-10 text-center text-[13px] text-red-300/80">
      {{ error }}
    </div>
    <div v-else class="flex-1 overflow-y-auto pb-3">
      <button
        v-for="(t, i) in tracks"
        :key="t.id"
        class="flex w-full items-center gap-2.5 px-5 py-[7px] text-left transition-colors hover:bg-white/[0.04]"
        @click="$emit('play-track', i)"
      >
        <span class="w-6 shrink-0 text-right text-[11px] text-white/30">{{ i + 1 }}</span>
        <div class="min-w-0 flex-1">
          <div class="truncate text-[12.5px] text-white/85">{{ t.name }}</div>
          <div class="mt-px text-[11px] text-[rgba(255,255,255,.38)]">{{ t.artist }}</div>
        </div>
      </button>
    </div>
  </div>
</template>
