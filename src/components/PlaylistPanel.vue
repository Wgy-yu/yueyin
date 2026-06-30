<script setup lang="ts">
import { onMounted } from "vue";
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

function switchSource(src: SourceType) {
  account.activeSource = src;
  account.fetchPlaylists(src);
}

onMounted(() => {
  if (!account.playlists.length) account.fetchPlaylists();
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
  <div class="fixed inset-0 z-[90] flex justify-end bg-black/50 backdrop-blur-md" @click.self="emit('close')">
    <div class="flex h-full w-[min(420px,88vw)] flex-col overflow-hidden border-l border-white/[0.06] bg-gradient-to-b from-[rgba(22,26,32,.98)] to-[rgba(10,12,16,.99)]">
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
