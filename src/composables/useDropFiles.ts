import { onUnmounted } from "vue";
import { useQueueStore } from "../stores/queue";
import { usePlayerStore } from "../stores/player";
import type { Track } from "../types/track";

const AUDIO_EXTS = new Set(["mp3", "flac", "wav", "ogg", "m4a", "aac", "wma", "opus", "webm"]);

function isAudioFile(name: string): boolean {
  const ext = name.split(".").pop()?.toLowerCase() ?? "";
  return AUDIO_EXTS.has(ext);
}

export function useDropFiles(container: HTMLElement) {
  const queue = useQueueStore();
  const player = usePlayerStore();

  const onDragOver = (e: DragEvent) => {
    e.preventDefault();
    if (e.dataTransfer) e.dataTransfer.dropEffect = "copy";
  };

  const onDrop = (e: DragEvent) => {
    e.preventDefault();
    const files = e.dataTransfer?.files;
    if (!files?.length) return;

    const audioFiles = Array.from(files).filter((f) => isAudioFile(f.name));
    if (!audioFiles.length) return;

    // ponytail: create object URLs directly, no file reader needed
    for (const file of audioFiles) {
      const url = URL.createObjectURL(file);
      const track: Track = {
        id: `local:${file.name}:${file.size}`,
        name: file.name.replace(/\.[^.]+$/, ""),
        artist: "本地文件",
        source: "local",
        duration: undefined,
        coverUrl: undefined,
        extra: { blobUrl: url },
      };
      queue.add(track);
    }

    // Start playing if nothing is playing
    if (!player.playing && queue.tracks.length) {
      queue.currentIndex = queue.tracks.length - audioFiles.length;
      player.playing = true;
    }
  };

  container.addEventListener("dragover", onDragOver);
  container.addEventListener("drop", onDrop);

  onUnmounted(() => {
    container.removeEventListener("dragover", onDragOver);
    container.removeEventListener("drop", onDrop);
  });
}
