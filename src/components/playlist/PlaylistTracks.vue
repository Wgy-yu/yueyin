<script setup lang="ts">
import type { Track } from "../../types/track";

defineProps<{ tracks: Track[]; error?: string }>();
defineEmits<{ (e: "play-all"): void; (e: "play-track", index: number): void }>();
</script>

<template>
  <div class="track-detail">
    <div class="track-detail-head">
      <span>{{ tracks.length }} 首歌曲</span>
      <button @click="$emit('play-all')">播放全部</button>
    </div>
    <div v-if="error" class="track-empty text-red-300/80">
      {{ error }}
    </div>
    <div v-else class="track-list">
      <button
        v-for="(t, i) in tracks"
        :key="t.id"
        class="track-row"
        :style="{ '--i': i }"
        @click="$emit('play-track', i)"
      >
        <span class="track-index">{{ i + 1 }}</span>
        <div class="track-meta">
          <div class="track-title">{{ t.name }}</div>
          <div class="track-artist">{{ t.artist }}</div>
        </div>
      </button>
    </div>
  </div>
</template>

<style scoped>
.track-detail {
  display: flex;
  flex: 1;
  min-height: 0;
  flex-direction: column;
  overflow: hidden;
}

.track-detail-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 24px 32px 14px;
  color: rgba(255, 255, 255, 0.48);
  font-size: 13px;
  font-weight: 720;
}

.track-detail-head button {
  height: 34px;
  padding: 0 20px;
  border-radius: 10px;
  border: 1px solid rgba(0, 245, 212, 0.32);
  background: rgba(0, 245, 212, 0.08);
  color: rgba(0, 245, 212, 0.86);
  font-size: 12px;
  font-weight: 820;
  transition: transform 0.22s, background 0.22s, box-shadow 0.22s;
}

.track-detail-head button:hover {
  transform: translateY(-1px);
  background: rgba(0, 245, 212, 0.14);
  box-shadow: 0 0 22px rgba(0, 245, 212, 0.12);
}

.track-empty {
  padding: 72px 20px;
  text-align: center;
  font-size: 13px;
}

.track-list {
  flex: 1;
  overflow-y: auto;
  padding: 4px 26px 44px;
  scrollbar-width: thin;
  scrollbar-color: rgba(255, 255, 255, 0.16) transparent;
}

.track-list::-webkit-scrollbar {
  width: 4px;
}

.track-list::-webkit-scrollbar-thumb {
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.16);
}

.track-row {
  display: grid;
  grid-template-columns: 44px minmax(0, 1fr);
  width: 100%;
  align-items: center;
  gap: 10px;
  padding: 9px 10px;
  border-radius: 12px;
  color: rgba(255, 255, 255, 0.86);
  text-align: left;
  transform: translateY(0);
  animation: track-rise 0.46s cubic-bezier(0.16, 1, 0.3, 1) both;
  animation-delay: calc(min(var(--i), 16) * 22ms);
  transition: background 0.18s, transform 0.18s;
}

.track-row:hover {
  background: rgba(255, 255, 255, 0.055);
  transform: translateX(-4px);
}

.track-index {
  color: rgba(255, 255, 255, 0.28);
  text-align: right;
  font-size: 12px;
  font-weight: 740;
}

.track-meta {
  min-width: 0;
}

.track-title {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 14px;
  font-weight: 720;
}

.track-artist {
  margin-top: 3px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  color: rgba(255, 255, 255, 0.38);
  font-size: 12px;
  font-weight: 650;
}

@keyframes track-rise {
  from {
    opacity: 0;
    transform: translateY(12px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>
