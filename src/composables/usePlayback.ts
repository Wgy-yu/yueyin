import { watch } from "vue";
import { storeToRefs } from "pinia";
import { AudioEngine } from "../services/audio-engine";
import { getSongUrl, proxiedAudioUrl } from "../services/music";
import { usePlayerStore } from "../stores/player";
import { useQueueStore } from "../stores/queue";

let engine: AudioEngine | null = null;

export function usePlayback() {
  const player = usePlayerStore();
  const queue = useQueueStore();
  const { playing, volume, currentTrack: playerTrack } = storeToRefs(player);

  function getEngine() {
    if (!engine) {
      engine = new AudioEngine();
      engine.on("timeupdate", () => { player.currentTime = engine!.currentTime; });
      engine.on("loadedmetadata", () => { player.duration = engine!.duration; });
      engine.on("ended", () => {
        if (player.playMode === "single") {
          engine!.seek(0);
          engine!.play();
        } else {
          queue.currentIndex = queue.nextIndex(player.playMode);
        }
      });
    }
    return engine;
  }

  // React to queue index changes → load and play
  watch(() => queue.currentIndex, async () => {
    const track = queue.currentTrack();
    if (!track) return;
    playerTrack.value = track;
    player.currentTime = 0;
    player.duration = 0;

    const url = await getSongUrl(track.id, track.source);
    if (!url) { console.warn("无法获取播放地址:", track.name); return; }

    const eng = getEngine();
    await eng.load(proxiedAudioUrl(url));
    if (playing.value) await eng.play();
  });

  // React to play/pause toggle
  watch(playing, async (val) => {
    const eng = getEngine();
    if (val) {
      if (!eng.paused || queue.currentTrack()) await eng.play();
    } else {
      eng.pause();
    }
  });

  // React to volume changes
  watch(volume, (val) => {
    engine?.setVolume(val);
  });
}
