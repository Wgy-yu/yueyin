<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import { gsap } from "gsap";
import { useAccountStore } from "../stores/account";
import { useQueueStore } from "../stores/queue";
import { usePlayerStore } from "../stores/player";
import type { SourceType } from "../types/track";
import PlaylistHeader from "./playlist/PlaylistHeader.vue";
import PlaylistList from "./playlist/PlaylistList.vue";
import PlaylistSourceTabs from "./playlist/PlaylistSourceTabs.vue";
import PlaylistTracks from "./playlist/PlaylistTracks.vue";

const emit = defineEmits<{ (e: "close"): void }>();
const account = useAccountStore();
const queue = useQueueStore();
const player = usePlayerStore();
const panelRef = ref<HTMLElement | null>(null);
let ctx: gsap.Context | null = null;

function switchSource(src: SourceType) {
  account.activeSource = src;
  account.fetchPlaylists(src);
}

onMounted(() => {
  if (!account.playlists.length) account.fetchPlaylists();
  if (!panelRef.value) return;
  ctx = gsap.context(() => {
    const reduceMotion = matchMedia("(prefers-reduced-motion: reduce)").matches;
    gsap.from(".shelf-shell", {
      x: reduceMotion ? 0 : 80,
      autoAlpha: 0,
      duration: reduceMotion ? 0 : 0.58,
      ease: "power3.out",
    });
    gsap.from(".shelf-card", {
      x: reduceMotion ? 0 : 64,
      rotationY: reduceMotion ? 0 : -18,
      autoAlpha: 0,
      duration: reduceMotion ? 0 : 0.62,
      stagger: { each: 0.045, from: "start" },
      ease: "power3.out",
      delay: reduceMotion ? 0 : 0.08,
    });
  }, panelRef.value);
});

onUnmounted(() => {
  ctx?.revert();
});

async function openPlaylist(id: string) {
  await account.fetchPlaylistTracks(id);
}

async function playAll() {
  if (!account.playlistTracks.length) return;
  queue.clear();
  for (const t of account.playlistTracks) queue.add(t);
  queue.currentIndex = 0;
  player.playing = true;
}

async function playTrack(idx: number) {
  for (const t of account.playlistTracks) queue.add(t);
  queue.currentIndex = idx;
  player.playing = true;
}

function back() {
  account.playlistTracks = [];
}
</script>

<template>
  <div ref="panelRef" class="shelf-overlay" @click.self="emit('close')">
    <div class="shelf-vignette"></div>
    <div class="shelf-shell">
      <PlaylistHeader :has-tracks="!!account.playlistTracks.length" @back="back" @close="emit('close')" />

      <PlaylistSourceTabs v-if="!account.playlistTracks.length" :active-source="account.activeSource" @switch="switchSource" />

      <div v-if="account.playlistLoading" class="p-10 text-center text-[13px] text-white/40">加载中...</div>

      <PlaylistList
        v-else-if="!account.playlistTracks.length"
        :playlists="account.playlists"
        :logged-in="account.current().loggedIn"
        :error="account.playlistError"
        @open="openPlaylist"
      />
      <PlaylistTracks v-else :tracks="account.playlistTracks" :error="account.playlistError" @play-all="playAll" @play-track="playTrack" />
    </div>
  </div>
</template>

<style scoped>
.shelf-overlay {
  position: fixed;
  inset: 0;
  z-index: 90;
  display: flex;
  justify-content: flex-end;
  overflow: hidden;
  background:
    radial-gradient(circle at 78% 42%, rgba(0, 245, 212, 0.1), transparent 24%),
    linear-gradient(90deg, rgba(0, 0, 0, 0.76), rgba(0, 0, 0, 0.28) 55%, rgba(0, 0, 0, 0.14));
  backdrop-filter: blur(18px) saturate(1.08);
  -webkit-backdrop-filter: blur(18px) saturate(1.08);
}

.shelf-vignette {
  position: absolute;
  inset: 0;
  pointer-events: none;
  background:
    linear-gradient(90deg, rgba(0, 0, 0, 0.72), transparent 46%),
    radial-gradient(circle at 35% 50%, rgba(255, 255, 255, 0.06), transparent 22%);
}

.shelf-shell {
  position: relative;
  display: flex;
  height: 100%;
  width: min(650px, 42vw);
  min-width: 520px;
  flex-direction: column;
  overflow: hidden;
  border-left: 1px solid rgba(255, 255, 255, 0.07);
  background:
    linear-gradient(180deg, rgba(23, 26, 32, 0.78), rgba(6, 8, 12, 0.9)),
    radial-gradient(circle at 24% 18%, rgba(255, 255, 255, 0.08), transparent 28%);
  box-shadow: -34px 0 120px rgba(0, 0, 0, 0.45), inset 1px 0 0 rgba(255, 255, 255, 0.04);
  backdrop-filter: blur(36px) saturate(1.2);
  -webkit-backdrop-filter: blur(36px) saturate(1.2);
}

.shelf-shell::before {
  content: "";
  position: absolute;
  inset: 0;
  pointer-events: none;
  background:
    linear-gradient(90deg, rgba(255, 255, 255, 0.028) 0 1px, transparent 1px 46px),
    linear-gradient(0deg, rgba(255, 255, 255, 0.02) 0 1px, transparent 1px 44px);
  opacity: 0.42;
}

.shelf-shell > :deep(*) {
  position: relative;
  z-index: 1;
}

@media (max-width: 980px) {
  .shelf-shell {
    width: min(620px, 76vw);
    min-width: 0;
  }
}

@media (max-width: 680px) {
  .shelf-shell {
    width: 100vw;
  }
}
</style>
