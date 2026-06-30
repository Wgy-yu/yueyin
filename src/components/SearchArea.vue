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
  <div id="search-area" :class="{ peek: true }">
    <div id="search-stack">
      <div id="search-box" :class="{ focused: isFocused }">
        <svg
          id="search-icon"
          width="17"
          height="17"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          viewBox="0 0 24 24"
        >
          <circle cx="11" cy="11" r="8" />
          <path d="m21 21-4.35-4.35" />
        </svg>
        <input
          id="search-input"
          v-model="localQuery"
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
      <div v-if="results.length || loading || error" id="search-results">
        <div v-if="loading" class="search-status">搜索中...</div>
        <div v-else-if="error" class="search-status error">{{ error }}</div>
        <ul v-else class="result-list">
          <li
            v-for="track in results.slice(0, 20)"
            :key="track.source + ':' + track.id"
            class="result-item"
            @mousedown.prevent="playTrack(track)"
          >
            <span class="result-name">{{ track.name }}</span>
            <span class="result-artist">{{ track.artist }}</span>
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>

<style scoped>
#search-area {
  position: fixed;
  z-index: 10;
  top: -76px;
  left: 50%;
  transform: translateX(-50%);
  width: auto;
  display: flex;
  align-items: flex-start;
  gap: 12px;
  transition: top 0.45s cubic-bezier(0.2, 0.7, 0.2, 1), opacity 0.35s,
    transform 0.35s cubic-bezier(0.2, 0.7, 0.2, 1);
  opacity: 0;
  pointer-events: none;
}

#search-area.peek {
  top: 24px;
  opacity: 1;
  pointer-events: auto;
}

#search-stack {
  width: min(520px, 58vw);
}

#search-box {
  display: flex;
  align-items: center;
  height: 58px;
  border-radius: 22px;
  padding: 0 20px;
  background: linear-gradient(
    112deg,
    rgba(72, 74, 76, 0.62),
    rgba(24, 27, 30, 0.7) 48%,
    rgba(8, 12, 14, 0.74)
  );
  border: 1px solid rgba(0, 245, 212, 0.3);
  box-shadow: 0 22px 64px rgba(0, 0, 0, 0.3),
    0 0 34px rgba(0, 245, 212, 0.052),
    inset 0 1px 0 rgba(255, 255, 255, 0.16),
    inset 0 -24px 58px rgba(0, 0, 0, 0.16);
  backdrop-filter: blur(34px) saturate(1.34);
  -webkit-backdrop-filter: blur(34px) saturate(1.34);
  transition: border-color 0.28s, background 0.28s, box-shadow 0.28s,
    transform 0.28s;
  cursor: text;
}

#search-box:focus-within {
  border-color: rgba(0, 245, 212, 0.5);
  background: linear-gradient(
    112deg,
    rgba(88, 91, 92, 0.68),
    rgba(28, 32, 35, 0.76) 50%,
    rgba(8, 13, 15, 0.82)
  );
  box-shadow: 0 24px 72px rgba(0, 0, 0, 0.34),
    0 0 0 1px rgba(0, 245, 212, 0.13), 0 0 42px rgba(0, 245, 212, 0.075),
    inset 0 1px 0 rgba(255, 255, 255, 0.2);
  transform: translateY(-1px);
}

#search-icon {
  color: rgba(255, 255, 255, 0.3);
  margin-right: 10px;
  flex-shrink: 0;
}

#search-input {
  flex: 1;
  border: none;
  background: none;
  color: #fff;
  font-size: 13.5px;
  font-family: inherit;
  outline: none;
  letter-spacing: 0.3px;
}

#search-input::placeholder {
  color: rgba(255, 255, 255, 0.22);
}

#search-results {
  margin-top: 6px;
  max-height: 360px;
  overflow-y: auto;
  border-radius: 16px;
  background: rgba(12, 14, 16, 0.88);
  backdrop-filter: blur(24px);
  -webkit-backdrop-filter: blur(24px);
  border: 1px solid rgba(255, 255, 255, 0.06);
  box-shadow: 0 16px 48px rgba(0, 0, 0, 0.4);
}

.search-status {
  padding: 16px 20px;
  font-size: 12px;
  color: rgba(255, 255, 255, 0.4);
  text-align: center;
}

.search-status.error {
  color: rgba(255, 100, 120, 0.8);
}

.result-list {
  list-style: none;
  margin: 0;
  padding: 6px 0;
}

.result-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 20px;
  cursor: pointer;
  transition: background 0.15s;
}

.result-item:hover {
  background: rgba(0, 245, 212, 0.06);
}

.result-name {
  font-size: 13px;
  color: rgba(255, 255, 255, 0.88);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  flex: 1;
  min-width: 0;
}

.result-artist {
  font-size: 11px;
  color: rgba(255, 255, 255, 0.35);
  white-space: nowrap;
  flex-shrink: 0;
}

@media (max-width: 720px) {
  #search-area.peek {
    top: 58px;
  }

  #search-stack {
    width: calc(100vw - 56px);
  }
}
</style>
