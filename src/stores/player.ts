import { defineStore } from "pinia";
import { ref, shallowRef } from "vue";
import type { Track, PlayMode } from "../types/track";
import { useSettingsStore } from "./settings";

export const usePlayerStore = defineStore("player", () => {
  const playing = ref(false);
  const currentTrack = shallowRef<Track | null>(null);
  const currentTime = ref(0);
  const duration = ref(0);
  const volume = ref(0.7);
  const playMode = ref<PlayMode>("loop");
  const muted = ref(false);
  let lastNonZeroVolume = 0.7;

  function init() {
    const settings = useSettingsStore();
    if (settings.loaded) {
      volume.value = parseFloat(settings.get("volume")) || 0.7;
      const savedMode = settings.get("playMode");
      if (savedMode === "loop" || savedMode === "shuffle" || savedMode === "single") {
        playMode.value = savedMode;
      }
    }
  }

  function setVolume(v: number) {
    volume.value = Math.max(0, Math.min(1, v));
    if (volume.value > 0) lastNonZeroVolume = volume.value;
    useSettingsStore().set("volume", String(volume.value));
  }

  function toggleMute() {
    if (muted.value) {
      muted.value = false;
      volume.value = lastNonZeroVolume;
    } else {
      lastNonZeroVolume = volume.value || lastNonZeroVolume;
      muted.value = true;
      volume.value = 0;
    }
  }

  function cyclePlayMode() {
    const modes: PlayMode[] = ["loop", "shuffle", "single"];
    const idx = modes.indexOf(playMode.value);
    playMode.value = modes[(idx + 1) % modes.length];
    useSettingsStore().set("playMode", playMode.value);
  }

  return {
    playing, currentTrack, currentTime, duration,
    volume, playMode, muted,
    init, setVolume, toggleMute, cyclePlayMode,
  };
});
