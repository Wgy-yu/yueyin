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
  <div v-if="lines.length || loading" class="pointer-events-none fixed right-7 top-[100px] z-[8] max-h-[calc(100vh-240px)] w-[min(380px,36vw)] overflow-hidden">
    <div v-if="loading" class="p-5 text-center text-[12px] text-white/30">加载歌词中...</div>
    <div v-else ref="containerRef" class="flex flex-col gap-1.5 py-10 [mask-image:linear-gradient(180deg,transparent_0%,#000_15%,#000_85%,transparent_100%)]">
      <div
        v-for="(line, i) in lines"
        :key="i"
        class="pr-2 text-right text-[14px] font-medium leading-[1.6] transition-all duration-300"
        :class="{
          'text-white/20': i !== currentIndex && i >= currentIndex,
          'text-white/12': i < currentIndex,
          'text-[rgba(0,245,212,.9)] text-[16px] font-bold scale-[1.02] shadow-[0_0_20px_rgba(0,245,212,.2)]': i === currentIndex,
        }"
        :style="i === currentIndex ? { '--progress': progressRatio } : undefined"
      >
        {{ line.text }}
      </div>
    </div>
  </div>
</template>
