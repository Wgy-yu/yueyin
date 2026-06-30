<script setup lang="ts">
import { onMounted } from "vue";
import { useAccountStore } from "../stores/account";
import { useQueueStore } from "../stores/queue";
import { usePlayerStore } from "../stores/player";
import type { SourceType } from "../types/track";

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
      <!-- Header -->
      <div class="flex items-center gap-2.5 border-b border-white/[0.06] px-5 py-[18px]">
        <button v-if="account.playlistTracks.length" class="rounded-md px-2 py-1 text-[13px] text-[rgba(0,245,212,.8)] transition-colors hover:bg-[rgba(0,245,212,.08)]" @click="back">← 返回</button>
        <div class="flex-1 text-[15px] font-bold text-white/90">{{ account.playlistTracks.length ? '歌单详情' : '我的歌单' }}</div>
        <button class="rounded-md px-2 py-1 text-xl text-white/50 transition-colors hover:bg-white/[0.08] hover:text-white" @click="emit('close')">×</button>
      </div>

      <!-- Source tabs -->
      <div v-if="!account.playlistTracks.length" class="flex gap-1 px-5 py-2.5">
        <button
          class="h-8 flex-1 rounded-lg border text-[12px] font-semibold transition-all"
          :class="account.activeSource === 'netease' ? 'border-[rgba(0,245,212,.3)] bg-[rgba(0,245,212,.12)] text-[rgba(0,245,212,.9)]' : 'border-white/[0.08] bg-white/[0.04] text-white/60'"
          @click="switchSource('netease')"
        >网易云</button>
        <button
          class="h-8 flex-1 rounded-lg border text-[12px] font-semibold transition-all"
          :class="account.activeSource === 'qq' ? 'border-[rgba(0,245,212,.3)] bg-[rgba(0,245,212,.12)] text-[rgba(0,245,212,.9)]' : 'border-white/[0.08] bg-white/[0.04] text-white/60'"
          @click="switchSource('qq')"
        >QQ</button>
      </div>

      <!-- Loading -->
      <div v-if="account.playlistLoading" class="p-10 text-center text-[13px] text-white/40">加载中...</div>

      <!-- Playlist list -->
      <div v-else-if="!account.playlistTracks.length" class="flex-1 overflow-y-auto py-2">
        <div v-if="!account.playlists.length" class="p-10 text-center text-[13px] text-white/40">
          {{ account.current().loggedIn ? '暂无歌单' : '请先登录' }}
        </div>
        <button
          v-for="pl in account.playlists"
          :key="pl.id"
          class="flex w-full items-center gap-3 px-5 py-2 text-left transition-colors hover:bg-white/[0.04]"
          @click="openPlaylist(pl.id)"
        >
          <div class="h-12 w-12 shrink-0 rounded-lg bg-gradient-to-br from-[rgba(0,245,212,.2)] to-[rgba(36,66,255,.2)] bg-cover bg-center" :style="pl.cover ? { backgroundImage: `url(${pl.cover})` } : undefined"></div>
          <div class="min-w-0 flex-1">
            <div class="truncate text-[13px] font-semibold text-white/90">{{ pl.name }}</div>
            <div class="mt-0.5 text-[11px] text-white/40">{{ pl.trackCount }} 首 · {{ pl.creator }}</div>
          </div>
        </button>
      </div>

      <!-- Track list -->
      <div v-else class="flex flex-1 flex-col overflow-hidden">
        <div class="flex items-center justify-between px-5 py-2.5 text-[12px] text-white/50">
          <span>{{ account.playlistTracks.length }} 首歌曲</span>
          <button class="rounded-lg border border-[rgba(0,245,212,.25)] bg-[rgba(0,245,212,.08)] px-3.5 py-1 text-[11px] font-semibold text-[rgba(0,245,212,.9)] transition-colors hover:bg-[rgba(0,245,212,.16)]" @click="playAll">播放全部</button>
        </div>
        <div class="flex-1 overflow-y-auto pb-3">
          <button
            v-for="(t, i) in account.playlistTracks"
            :key="t.id"
            class="flex w-full items-center gap-2.5 px-5 py-[7px] text-left transition-colors hover:bg-white/[0.04]"
            @click="playTrack(i)"
          >
            <span class="w-6 shrink-0 text-right text-[11px] text-white/30">{{ i + 1 }}</span>
            <div class="min-w-0 flex-1">
              <div class="truncate text-[12.5px] text-white/85">{{ t.name }}</div>
              <div class="mt-px text-[11px] text-[rgba(255,255,255,.38)]">{{ t.artist }}</div>
            </div>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
