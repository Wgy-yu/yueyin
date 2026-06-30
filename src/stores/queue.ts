import { defineStore } from "pinia";
import { ref } from "vue";
import type { Track } from "../types/track";

export const useQueueStore = defineStore("queue", () => {
  const tracks = ref<Track[]>([]);
  const currentIndex = ref(-1);

  const currentTrack = () =>
    currentIndex.value >= 0 && currentIndex.value < tracks.value.length
      ? tracks.value[currentIndex.value]
      : null;

  function add(track: Track, position?: "next" | "end") {
    const key = trackKey(track);
    const existingIdx = tracks.value.findIndex((t) => trackKey(t) === key);
    if (existingIdx >= 0) {
      if (position === "next" && existingIdx !== currentIndex.value) {
        const [item] = tracks.value.splice(existingIdx, 1);
        const insertAt = currentIndex.value >= 0 ? currentIndex.value + 1 : 0;
        tracks.value.splice(insertAt, 0, item);
        if (existingIdx < currentIndex.value) currentIndex.value--;
      }
      return existingIdx;
    }
    if (position === "next") {
      const insertAt = currentIndex.value >= 0 ? currentIndex.value + 1 : 0;
      tracks.value.splice(insertAt, 0, track);
      return insertAt;
    }
    tracks.value.push(track);
    return tracks.value.length - 1;
  }

  function remove(idx: number) {
    tracks.value.splice(idx, 1);
    if (idx < currentIndex.value) currentIndex.value--;
    else if (idx === currentIndex.value) {
      currentIndex.value = Math.min(currentIndex.value, tracks.value.length - 1);
    }
  }

  function clear() {
    tracks.value = [];
    currentIndex.value = -1;
  }

  function nextIndex(playMode: string): number {
    if (!tracks.value.length) return -1;
    if (playMode === "shuffle") {
      if (tracks.value.length === 1) return 0;
      let r;
      do { r = Math.floor(Math.random() * tracks.value.length); } while (r === currentIndex.value);
      return r;
    }
    return (currentIndex.value + 1) % tracks.value.length;
  }

  function prevIndex(): number {
    if (!tracks.value.length) return -1;
    return (currentIndex.value - 1 + tracks.value.length) % tracks.value.length;
  }

  return { tracks, currentIndex, currentTrack, add, remove, clear, nextIndex, prevIndex };
});

function trackKey(t: Track): string {
  return `${t.source}:${t.id}`;
}
