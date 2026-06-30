export type AudioEvent = "timeupdate" | "statechange" | "ended" | "loadedmetadata";

export interface AudioAnalysis {
  bass: number;
  mid: number;
  treble: number;
  energy: number;
}

export class AudioEngine {
  private audio: HTMLAudioElement | null = null;
  private ctx: AudioContext | null = null;
  private source: MediaElementAudioSourceNode | null = null;
  private gain: GainNode | null = null;
  private analyser: AnalyserNode | null = null;
  private freqData: Uint8Array | null = null;
  private listeners = new Map<AudioEvent, Set<() => void>>();
  private fadeTimer: ReturnType<typeof setTimeout> | null = null;
  private smoothBass = 0;
  private smoothMid = 0;
  private smoothTreble = 0;

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
    this.analyser = this.ctx.createAnalyser();
    this.analyser.fftSize = 2048;
    this.analyser.smoothingTimeConstant = 0.58;
    this.freqData = new Uint8Array(this.analyser.frequencyBinCount);
    this.gain = this.ctx.createGain();
    this.source.connect(this.analyser).connect(this.gain).connect(this.ctx.destination);
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

  getAnalysis(): AudioAnalysis {
    if (!this.analyser || !this.freqData) return { bass: 0, mid: 0, treble: 0, energy: 0 };
    // ponytail: TS strict Uint8Array generic mismatch with Web Audio API typings
    this.analyser.getByteFrequencyData(this.freqData as any);
    const d = this.freqData;
    // Bin ranges matching Mineradio: kick 0-7, vocal 7-140, mid 140-280, treble 280+
    let bass = 0, mid = 0, treble = 0, total = 0;
    for (let i = 0; i < d.length; i++) {
      const v = d[i] / 255;
      total += v;
      if (i < 8) bass += v;
      else if (i < 140) mid += v;
      else treble += v;
    }
    bass /= 8; mid /= 133; treble /= (d.length - 140);
    // Asymmetric smoothing: fast attack, slow release
    const attack = 0.3, release = 0.08;
    this.smoothBass += (bass - this.smoothBass) * (bass > this.smoothBass ? attack : release);
    this.smoothMid += (mid - this.smoothMid) * (mid > this.smoothMid ? attack : release);
    this.smoothTreble += (treble - this.smoothTreble) * (treble > this.smoothTreble ? attack : release);
    const energy = total / d.length;
    return { bass: this.smoothBass, mid: this.smoothMid, treble: this.smoothTreble, energy };
  }

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
