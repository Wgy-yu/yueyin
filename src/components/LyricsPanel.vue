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
  <div v-if="lines.length || loading" class="stage-lyrics pointer-events-none fixed z-[8] overflow-hidden">
    <div v-if="loading" class="p-5 text-center text-[12px] text-white/30">加载歌词中...</div>
    <div v-else ref="containerRef" class="stage-lyrics-list">
      <div
        v-for="(line, i) in lines"
        :key="i"
        class="stage-lyric-line"
        :class="{
          'stage-lyric-inactive': i !== currentIndex,
          'stage-lyric-past': i < currentIndex,
          'stage-lyric-active': i === currentIndex,
        }"
        :style="i === currentIndex ? { '--progress': progressRatio } : undefined"
      >
        {{ line.text }}
      </div>
    </div>
  </div>
</template>

<style scoped>
.stage-lyrics {
  left: 17%;
  right: 24%;
  top: 41%;
  height: 170px;
  mask-image: linear-gradient(180deg, transparent, #000 28%, #000 72%, transparent);
}
.stage-lyrics-list {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 42px;
  padding: 52px 0;
}
.stage-lyric-line {
  width: 100%;
  text-align: center;
  font-size: clamp(34px, 4.3vw, 74px);
  line-height: 1;
  font-weight: 900;
  letter-spacing: -0.045em;
  color: rgba(255,255,255,.1);
  transition: opacity .5s, transform .7s cubic-bezier(.16,1,.3,1), color .5s;
}
.stage-lyric-inactive { opacity: 0; transform: scale(.88); }
.stage-lyric-past { opacity: 0; }
.stage-lyric-active {
  opacity: 1;
  color: rgba(255,255,255,.96);
  transform: scale(1);
  text-shadow: 0 8px 38px rgba(0,0,0,.74), 0 0 20px rgba(255,255,255,.12);
}
</style>
