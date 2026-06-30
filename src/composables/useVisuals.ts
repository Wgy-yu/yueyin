import { watch, onUnmounted } from "vue";
import { storeToRefs } from "pinia";
import { usePlayerStore } from "../stores/player";
import { VisualEngine } from "../services/visual-engine";
import type { AudioEngine } from "../services/audio-engine";

let visual: VisualEngine | null = null;
let rafId: number | null = null;

export function getVisualEngine(): VisualEngine | null { return visual; }

export function useVisuals(getEngine: () => AudioEngine, container: HTMLElement) {
  const player = usePlayerStore();
  const { playing, currentTrack } = storeToRefs(player);

  visual = new VisualEngine();
  visual.mount(container);

  // Feed audio analysis to visuals every frame
  const tick = () => {
    rafId = requestAnimationFrame(tick);
    if (!playing.value) return;
    const analysis = getEngine().getAnalysis();
    // ponytail: simple beat proxy — bass spike above threshold
    const beat = analysis.bass > 0.6 ? (analysis.bass - 0.6) * 2.5 : 0;
    visual!.update(analysis, beat);
  };
  tick();

  // Update cover when track changes
  watch(currentTrack, (track) => {
    if (track?.coverUrl) visual?.setCover(track.coverUrl);
  });

  onUnmounted(() => {
    if (rafId) { cancelAnimationFrame(rafId); rafId = null; }
    visual?.dispose();
    visual = null;
  });
}
