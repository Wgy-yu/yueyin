import { watch, type WatchStopHandle } from "vue";
import { storeToRefs } from "pinia";
import { usePlayerStore } from "../stores/player";
import { useAccountStore } from "../stores/account";
import { VisualEngine } from "../services/visual-engine";
import type { AudioEngine } from "../services/audio-engine";

let visual: VisualEngine | null = null;
let rafId: number | null = null;
let stopTrackWatch: WatchStopHandle | null = null;
let stopPlaylistWatch: WatchStopHandle | null = null;

export function getVisualEngine(): VisualEngine | null { return visual; }

export function useVisuals(getEngine: () => AudioEngine | null, container: HTMLElement) {
  if (visual) return visual;
  const player = usePlayerStore();
  const account = useAccountStore();
  const { playing, currentTrack } = storeToRefs(player);

  visual = new VisualEngine();
  visual.mount(container);

  // Feed audio analysis to visuals every frame
  const tick = () => {
    rafId = requestAnimationFrame(tick);
    if (!playing.value) return;
    const engine = getEngine();
    if (!engine) return;
    const analysis = engine.getAnalysis();
    visual!.update(analysis, analysis.beat);
  };
  tick();

  // Update cover when track changes
  stopTrackWatch?.();
  stopTrackWatch = watch(currentTrack, (track) => {
    if (track?.coverUrl) visual?.setCover(track.coverUrl);
  }, { immediate: true });
  stopPlaylistWatch?.();
  stopPlaylistWatch = watch(() => account.playlists, (playlists) => {
    visual?.setPlaylists(playlists);
  }, { immediate: true, deep: true });

  return visual;
}

export function disposeVisuals() {
  stopTrackWatch?.();
  stopTrackWatch = null;
  stopPlaylistWatch?.();
  stopPlaylistWatch = null;
  if (rafId) { cancelAnimationFrame(rafId); rafId = null; }
  visual?.dispose();
  visual = null;
}
