import { watch } from "vue";
import { storeToRefs } from "pinia";
import { AudioEngine } from "../services/audio-engine";
import { fetchAudioBlobUrl, getSongUrl, proxiedAudioUrl } from "../services/music";
import { usePlayerStore } from "../stores/player";
import { useQueueStore } from "../stores/queue";
import { useLyricsStore } from "../stores/lyrics";

let engine: AudioEngine | null = null;
let loadToken = 0;

export function getAudioEngine(): AudioEngine | null { return engine; }

export function usePlayback() {
  const player = usePlayerStore();
  const queue = useQueueStore();
  const lyrics = useLyricsStore();
  const { playing, volume, currentTrack: playerTrack } = storeToRefs(player);

  function getEngine() {
    if (!engine) {
      engine = new AudioEngine();
      engine.on("timeupdate", () => {
        player.currentTime = engine!.currentTime;
        lyrics.updateProgress(player.currentTime);
      });
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
    const token = ++loadToken;
    const track = queue.currentTrack();
    if (!track) return;
    playerTrack.value = track;
    player.currentTime = 0;
    player.duration = 0;
    lyrics.load(track); // fire-and-forget

    const eng = getEngine();

    // Local file: use blob URL directly
    if (track.source === "local" && track.extra?.blobUrl) {
      await eng.load(track.extra.blobUrl as string);
      if (token !== loadToken) return;
      if (playing.value) await eng.play();
      return;
    }

    const url = await getSongUrl(track.id, track.source);
    if (token !== loadToken) return;
    if (!url) { console.warn("无法获取播放地址:", track.name); return; }

    const blobUrl = await fetchAudioBlobUrl(url);
    if (token !== loadToken) {
      if (blobUrl) URL.revokeObjectURL(blobUrl);
      return;
    }
    await eng.load(blobUrl || proxiedAudioUrl(url));
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
