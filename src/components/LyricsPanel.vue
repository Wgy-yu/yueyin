<script setup lang="ts">
import { computed, watch, ref } from "vue";
import { storeToRefs } from "pinia";
import { useLyricsStore } from "../stores/lyrics";
import { usePlayerStore } from "../stores/player";

const lyrics = useLyricsStore();
const player = usePlayerStore();
const { lines, currentIndex, loading } = storeToRefs(lyrics);
const { currentTime } = storeToRefs(player);
const containerRef = ref<HTMLElement | null>(null);

watch(currentTime, (t) => { lyrics.updateProgress(t); });

// Auto-scroll to current line
watch(currentIndex, (idx) => {
  if (idx < 0 || !containerRef.value) return;
  const el = containerRef.value.children[idx] as HTMLElement | undefined;
  el?.scrollIntoView({ behavior: "smooth", block: "center" });
});

const progressRatio = computed(() => {
  const line = lyrics.currentLine;
  if (!line) return 0;
  const elapsed = currentTime.value - line.t;
  return Math.max(0, Math.min(1, elapsed / line.duration));
});
</script>

<template>
  <div v-if="lines.length || loading" id="lyrics-panel">
    <div v-if="loading" class="lyrics-status">加载歌词中...</div>
    <div v-else ref="containerRef" class="lyrics-scroll">
      <div
        v-for="(line, i) in lines"
        :key="i"
        class="lyrics-line"
        :class="{ active: i === currentIndex, past: i < currentIndex }"
        :style="i === currentIndex ? { '--progress': progressRatio } : undefined"
      >
        {{ line.text }}
      </div>
    </div>
  </div>
</template>

<style scoped>
#lyrics-panel {
  position: fixed;
  z-index: 8;
  top: 100px;
  right: 28px;
  width: min(380px, 36vw);
  max-height: calc(100vh - 240px);
  overflow: hidden;
  pointer-events: none;
}

.lyrics-status {
  padding: 20px;
  font-size: 12px;
  color: rgba(255, 255, 255, 0.3);
  text-align: center;
}

.lyrics-scroll {
  display: flex;
  flex-direction: column;
  gap: 6px;
  padding: 40px 0;
  mask-image: linear-gradient(180deg, transparent 0%, #000 15%, #000 85%, transparent 100%);
  -webkit-mask-image: linear-gradient(180deg, transparent 0%, #000 15%, #000 85%, transparent 100%);
}

.lyrics-line {
  font-size: 14px;
  font-weight: 500;
  line-height: 1.6;
  color: rgba(255, 255, 255, 0.2);
  transition: color 0.35s, transform 0.35s, font-size 0.35s;
  text-align: right;
  padding-right: 8px;
}

.lyrics-line.past {
  color: rgba(255, 255, 255, 0.12);
}

.lyrics-line.active {
  color: rgba(0, 245, 212, 0.9);
  font-size: 16px;
  font-weight: 700;
  transform: scale(1.02);
  text-shadow: 0 0 20px rgba(0, 245, 212, 0.2);
  background: linear-gradient(90deg, rgba(0, 245, 212, 0.95) calc(var(--progress, 0) * 100%), rgba(255, 255, 255, 0.25) calc(var(--progress, 0) * 100%));
  -webkit-background-clip: text;
  background-clip: text;
  -webkit-text-fill-color: transparent;
}
</style>
