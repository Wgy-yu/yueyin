import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke, isTauri } from "@tauri-apps/api/core";
import type { Track } from "../types/track";

interface QueueTrackDTO {
  track_id: string;
  track_name: string;
  artist: string;
  album: string | null;
  cover_url: string | null;
  duration: number | null;
  source: string;
  extra: string | null;
}

export const useQueueStore = defineStore("queue", () => {
  const tracks = ref<Track[]>([]);
  const currentIndex = ref(-1);
  let saveTimer: ReturnType<typeof setTimeout> | null = null;

  const currentTrack = () =>
    currentIndex.value >= 0 && currentIndex.value < tracks.value.length
      ? tracks.value[currentIndex.value]
      : null;

  function scheduleSave() {
    if (saveTimer) clearTimeout(saveTimer);
    saveTimer = setTimeout(() => { save(); }, 800);
  }

  function add(track: Track, position?: "next" | "end") {
    const key = trackKey(track);
    const existingIdx = tracks.value.findIndex((t) => trackKey(t) === key);
    if (existingIdx >= 0) {
      if (position === "next" && existingIdx !== currentIndex.value) {
        const [item] = tracks.value.splice(existingIdx, 1);
        const insertAt = currentIndex.value >= 0 ? currentIndex.value + 1 : 0;
        tracks.value.splice(insertAt, 0, item);
        if (existingIdx < currentIndex.value) currentIndex.value--;
        scheduleSave();
      }
      return existingIdx;
    }
    if (position === "next") {
      const insertAt = currentIndex.value >= 0 ? currentIndex.value + 1 : 0;
      tracks.value.splice(insertAt, 0, track);
    } else {
      tracks.value.push(track);
    }
    scheduleSave();
    return tracks.value.length - 1;
  }

  function remove(idx: number) {
    tracks.value.splice(idx, 1);
    if (idx < currentIndex.value) currentIndex.value--;
    else if (idx === currentIndex.value) {
      currentIndex.value = Math.min(currentIndex.value, tracks.value.length - 1);
    }
    scheduleSave();
  }

  function clear() {
    tracks.value = [];
    currentIndex.value = -1;
    scheduleSave();
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

  async function save() {
    if (!isTauri()) return;
    const dtos: QueueTrackDTO[] = tracks.value.map((t) => ({
      track_id: t.id,
      track_name: t.name,
      artist: t.artist,
      album: t.album ?? null,
      cover_url: t.coverUrl ?? null,
      duration: t.duration ?? null,
      source: t.source,
      extra: t.extra ? JSON.stringify(t.extra) : null,
    }));
    try {
      await invoke("save_queue", { tracks: dtos });
    } catch (e) {
      console.warn("保存队列失败:", e);
    }
  }

  async function load() {
    if (!isTauri()) return;
    try {
      const dtos = await invoke<QueueTrackDTO[]>("load_queue");
      tracks.value = dtos.map((d) => ({
        id: d.track_id,
        name: d.track_name,
        artist: d.artist,
        album: d.album ?? undefined,
        coverUrl: d.cover_url ?? undefined,
        duration: d.duration ?? undefined,
        source: d.source as Track["source"],
        extra: d.extra ? JSON.parse(d.extra) : undefined,
      }));
      if (tracks.value.length > 0 && currentIndex.value < 0) {
        currentIndex.value = 0;
      }
    } catch (e) {
      console.warn("加载队列失败:", e);
    }
  }

  return {
    tracks, currentIndex, currentTrack,
    add, remove, clear, nextIndex, prevIndex,
    save, load,
  };
});

function trackKey(t: Track): string {
  return `${t.source}:${t.id}`;
}
