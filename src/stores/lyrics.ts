import { defineStore } from "pinia";
import { ref, computed } from "vue";
import type { LyricLine } from "../utils/lrc";
import { parseLrc, parsePlainText } from "../utils/lrc";
import { fetchLyrics } from "../services/music";
import type { Track } from "../types/track";

// ponytail: in-memory LRU cache, no new SQLite table
const cache = new Map<string, string>();
const CACHE_MAX = 200;

export const useLyricsStore = defineStore("lyrics", () => {
  const lines = ref<LyricLine[]>([]);
  const currentIndex = ref(-1);
  const loading = ref(false);
  const source = ref<"lrc" | "plain" | null>(null);

  const currentLine = computed(() =>
    currentIndex.value >= 0 ? lines.value[currentIndex.value] : null
  );

  async function load(track: Track) {
    const cacheKey = `${track.source}:${track.id}`;
    loading.value = true;
    lines.value = [];
    currentIndex.value = -1;
    source.value = null;
    try {
      let raw = cache.get(cacheKey);
      if (!raw) {
        raw = await fetchLyrics(track.id, track.source) ?? undefined;
        if (raw) {
          cache.set(cacheKey, raw);
          if (cache.size > CACHE_MAX) {
            // Delete oldest entry
            const first = cache.keys().next().value;
            if (first) cache.delete(first);
          }
        }
      }
      if (raw) {
        const parsed = parseLrc(raw);
        if (parsed.length > 1) {
          lines.value = parsed;
          source.value = "lrc";
        }
      }
      if (!lines.value.length && track.name) {
        lines.value = parsePlainText(track.name, track.duration ?? 240);
        source.value = "plain";
      }
    } catch {
      // Silent fail
    } finally {
      loading.value = false;
    }
  }

  function updateProgress(currentTime: number) {
    if (!lines.value.length) return;
    let lo = 0;
    let hi = lines.value.length - 1;
    let result = -1;
    while (lo <= hi) {
      const mid = (lo + hi) >>> 1;
      if (lines.value[mid].t <= currentTime + 0.05) {
        result = mid;
        lo = mid + 1;
      } else {
        hi = mid - 1;
      }
    }
    currentIndex.value = result;
  }

  function clear() {
    lines.value = [];
    currentIndex.value = -1;
    source.value = null;
  }

  return {
    lines, currentIndex, currentLine, loading, source,
    load, updateProgress, clear,
  };
});
