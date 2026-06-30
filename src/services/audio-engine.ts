export type AudioEvent = "timeupdate" | "statechange" | "ended" | "loadedmetadata";

export class AudioEngine {
  private audio: HTMLAudioElement | null = null;
  private ctx: AudioContext | null = null;
  private source: MediaElementAudioSourceNode | null = null;
  private gain: GainNode | null = null;
  private listeners = new Map<AudioEvent, Set<() => void>>();
  private fadeTimer: ReturnType<typeof setTimeout> | null = null;

  private ensureAudio(): HTMLAudioElement {
    if (!this.audio) {
      this.audio = new Audio();
      this.audio.crossOrigin = "anonymous";
      this.audio.addEventListener("timeupdate", () => this.emit("timeupdate"));
      this.audio.addEventListener("ended", () => this.emit("ended"));
      this.audio.addEventListener("loadedmetadata", () => this.emit("loadedmetadata"));
      this.audio.addEventListener("play", () => this.emit("statechange"));
      this.audio.addEventListener("pause", () => this.emit("statechange"));
    }
    return this.audio;
  }

  private ensureContext() {
    if (this.ctx) return;
    const audio = this.ensureAudio();
    this.ctx = new AudioContext();
    this.source = this.ctx.createMediaElementSource(audio);
    this.gain = this.ctx.createGain();
    this.source.connect(this.gain).connect(this.ctx.destination);
  }

  on(event: AudioEvent, fn: () => void) {
    if (!this.listeners.has(event)) this.listeners.set(event, new Set());
    this.listeners.get(event)!.add(fn);
  }

  off(event: AudioEvent, fn: () => void) {
    this.listeners.get(event)?.delete(fn);
  }

  private emit(event: AudioEvent) {
    this.listeners.get(event)?.forEach((fn) => fn());
  }

  get currentTime(): number { return this.audio?.currentTime ?? 0; }
  get duration(): number { return this.audio?.duration ?? 0; }
  get paused(): boolean { return this.audio?.paused ?? true; }

  async load(url: string) {
    const audio = this.ensureAudio();
    audio.src = url;
    audio.load();
  }

  async play() {
    this.ensureContext();
    if (this.ctx?.state === "suspended") await this.ctx.resume();
    const audio = this.ensureAudio();
    // ponytail: fade-in via gain ramp, 460ms matching Mineradio
    if (this.gain) {
      this.gain.gain.setValueAtTime(0, this.ctx!.currentTime);
      this.gain.gain.linearRampToValueAtTime(1, this.ctx!.currentTime + 0.46);
    }
    await audio.play().catch(() => {});
  }

  pause() {
    if (!this.audio) return;
    // ponytail: fade-out 420ms then pause
    if (this.gain && this.ctx) {
      this.gain.gain.setValueAtTime(this.gain.gain.value, this.ctx.currentTime);
      this.gain.gain.linearRampToValueAtTime(0, this.ctx.currentTime + 0.42);
      if (this.fadeTimer) clearTimeout(this.fadeTimer);
      this.fadeTimer = setTimeout(() => { this.audio?.pause(); }, 420);
    } else {
      this.audio.pause();
    }
  }

  seek(seconds: number) {
    if (this.audio) this.audio.currentTime = seconds;
  }

  setVolume(v: number) {
    this.ensureContext();
    if (this.gain) this.gain.gain.setValueAtTime(v, this.ctx!.currentTime);
  }

  dispose() {
    if (this.fadeTimer) { clearTimeout(this.fadeTimer); this.fadeTimer = null; }
    this.audio?.pause();
    this.audio = null;
    this.source = null;
    this.gain = null;
    this.ctx?.close().catch(() => {});
    this.ctx = null;
    this.listeners.clear();
  }
}
