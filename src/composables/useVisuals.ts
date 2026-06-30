import { onUnmounted } from "vue";
import { storeToRefs } from "pinia";
import { usePlayerStore } from "../stores/player";
import { VisualEngine } from "../services/visual-engine";
import type { AudioEngine } from "../services/audio-engine";

let visual: VisualEngine | null = null;
let rafId: number | null = null;

export function useVisuals(getEngine: () => AudioEngine, container: HTMLElement) {
  const player = usePlayerStore();
  const { playing } = storeToRefs(player);

  visual = new VisualEngine();
  visual.mount(container);

  // Feed audio analysis to visuals every frame
  const tick = () => {
    rafId = requestAnimationFrame(tick);
    if (!playing.value) return;
    const analysis = getEngine().getAnalysis();
    visual!.update(analysis);
  };
  tick();

  onUnmounted(() => {
    if (rafId) { cancelAnimationFrame(rafId); rafId = null; }
    visual?.dispose();
    visual = null;
  });
}
