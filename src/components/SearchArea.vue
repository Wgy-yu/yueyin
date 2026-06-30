<script setup lang="ts">
import { ref } from "vue";
import { storeToRefs } from "pinia";
import { useSearchStore } from "../stores/search";
import { useQueueStore } from "../stores/queue";
import { usePlayerStore } from "../stores/player";
import type { Track } from "../types/track";

const searchStore = useSearchStore();
const queueStore = useQueueStore();
const playerStore = usePlayerStore();
const { results, loading, error } = storeToRefs(searchStore);
const isFocused = ref(false);
const localQuery = ref("");

function onInput() {
  searchStore.debouncedSearch(localQuery.value);
}

function onKeydown(e: KeyboardEvent) {
  if (e.key === "Enter") searchStore.search(localQuery.value);
}

function playTrack(track: Track) {
  queueStore.add(track, "next");
  queueStore.currentIndex = queueStore.tracks.findIndex(
    (t) => t.id === track.id && t.source === track.source
  );
  playerStore.playing = true;
}
</script>

<template>
  <div class="fixed left-1/2 z-10 -translate-x-1/2 transition-all duration-[450ms] [transition-timing-function:cubic-bezier(.2,.7,.2,1)]" :class="isFocused || results.length || loading || error ? 'top-6 opacity-100 pointer-events-auto' : '-top-[76px] opacity-0 pointer-events-none'">
    <div class="w-[min(520px,58vw)]">
      <!-- Search box: glass-panel style -->
      <div
        class="flex h-[58px] items-center rounded-[22px] border px-5 transition-all duration-[280ms]"
        :class="isFocused
          ? 'border-[rgba(0,245,212,.5)] bg-[var(--glass-bg-focus)] shadow-[var(--glass-shadow-focus)] -translate-y-px'
          : 'border-[rgba(0,245,212,.3)] bg-[var(--glass-bg)] shadow-glass'"
      >
        <svg class="mr-2.5 shrink-0 text-white/30" width="17" height="17" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
          <circle cx="11" cy="11" r="8" />
          <path d="m21 21-4.35-4.35" />
        </svg>
        <input
          v-model="localQuery"
          class="flex-1 bg-transparent text-[13.5px] tracking-[0.3px] text-white outline-none placeholder:text-white/[0.22]"
          type="text"
          placeholder="搜索歌曲、歌手..."
          autocomplete="off"
          spellcheck="false"
          @focus="isFocused = true"
          @blur="isFocused = false"
          @input="onInput"
          @keydown="onKeydown"
        />
      </div>

      <!-- Results dropdown -->
      <div v-if="results.length || loading || error" class="mt-1.5 max-h-[360px] overflow-y-auto rounded-[14px] border border-white/[0.06] bg-[rgba(12,14,16,.88)] shadow-[0_16px_48px_rgba(0,0,0,.4)] backdrop-blur-6xl">
        <div v-if="loading" class="p-4 text-center text-[12px] text-white/40">搜索中...</div>
        <div v-else-if="error" class="p-4 text-center text-[12px] text-[rgba(255,100,120,.8)]">{{ error }}</div>
        <ul v-else class="py-1.5">
          <li
            v-for="track in results.slice(0, 20)"
            :key="track.source + ':' + track.id"
            class="flex cursor-pointer items-center gap-2.5 px-5 py-2.5 transition-colors hover:bg-[rgba(0,245,212,.06)]"
            @mousedown.prevent="playTrack(track)"
          >
            <span class="min-w-0 flex-1 truncate text-[13px] text-white/90">{{ track.name }}</span>
            <span class="shrink-0 text-[11px] text-white/35">{{ track.artist }}</span>
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>
