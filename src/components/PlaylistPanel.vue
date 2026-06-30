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
  <div class="pl-overlay" @click.self="emit('close')">
    <div class="pl-panel">
      <div class="pl-header">
        <button v-if="account.playlistTracks.length" class="pl-back" @click="back">← 返回</button>
        <div class="pl-title">{{ account.playlistTracks.length ? '歌单详情' : '我的歌单' }}</div>
        <button class="pl-close" @click="emit('close')">×</button>
      </div>

      <div v-if="!account.playlistTracks.length" class="pl-sources">
        <button :class="{ active: account.activeSource === 'netease' }" @click="switchSource('netease')">网易云</button>
        <button :class="{ active: account.activeSource === 'qq' }" @click="switchSource('qq')">QQ</button>
      </div>

      <div v-if="account.playlistLoading" class="pl-loading">加载中...</div>

      <div v-else-if="!account.playlistTracks.length" class="pl-list">
        <div v-if="!account.playlists.length" class="pl-empty">
          {{ account.current().loggedIn ? '暂无歌单' : '请先登录' }}
        </div>
        <button
          v-for="pl in account.playlists"
          :key="pl.id"
          class="pl-item"
          @click="openPlaylist(pl.id)"
        >
          <div class="pl-item-cover" :style="pl.cover ? { backgroundImage: `url(${pl.cover})` } : undefined"></div>
          <div class="pl-item-info">
            <div class="pl-item-name">{{ pl.name }}</div>
            <div class="pl-item-meta">{{ pl.trackCount }} 首 · {{ pl.creator }}</div>
          </div>
        </button>
      </div>

      <div v-else class="pl-tracks">
        <div class="pl-tracks-header">
          <span>{{ account.playlistTracks.length }} 首歌曲</span>
          <button class="pl-play-all" @click="playAll">播放全部</button>
        </div>
        <div class="pl-track-list">
          <button
            v-for="(t, i) in account.playlistTracks"
            :key="t.id"
            class="pl-track"
            @click="playTrack(i)"
          >
            <span class="pl-track-idx">{{ i + 1 }}</span>
            <div class="pl-track-info">
              <div class="pl-track-name">{{ t.name }}</div>
              <div class="pl-track-artist">{{ t.artist }}</div>
            </div>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.pl-overlay {
  position: fixed;
  inset: 0;
  z-index: 90;
  background: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(6px);
  display: flex;
  justify-content: flex-end;
}

.pl-panel {
  width: min(420px, 88vw);
  height: 100%;
  background: linear-gradient(170deg, rgba(22, 26, 32, 0.98), rgba(10, 12, 16, 0.99));
  border-left: 1px solid rgba(255, 255, 255, 0.06);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.pl-header {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 18px 20px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.06);
}

.pl-back {
  background: none;
  border: none;
  color: rgba(0, 245, 212, 0.8);
  font-size: 13px;
  cursor: pointer;
  padding: 4px 8px;
  border-radius: 6px;
  font-family: inherit;
}

.pl-back:hover { background: rgba(0, 245, 212, 0.08); }

.pl-title {
  flex: 1;
  font-size: 15px;
  font-weight: 720;
  color: rgba(255, 255, 255, 0.9);
}

.pl-close {
  background: none;
  border: none;
  color: rgba(255, 255, 255, 0.5);
  font-size: 20px;
  cursor: pointer;
  padding: 4px 8px;
  border-radius: 6px;
}

.pl-close:hover { background: rgba(255, 255, 255, 0.08); color: #fff; }

.pl-sources {
  display: flex;
  gap: 4px;
  padding: 10px 20px;
}

.pl-sources button {
  flex: 1;
  height: 32px;
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 8px;
  background: rgba(255, 255, 255, 0.04);
  color: rgba(255, 255, 255, 0.6);
  font-family: inherit;
  font-size: 12px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.15s;
}

.pl-sources button.active {
  background: rgba(0, 245, 212, 0.12);
  border-color: rgba(0, 245, 212, 0.3);
  color: rgba(0, 245, 212, 0.9);
}

.pl-loading, .pl-empty {
  padding: 40px 20px;
  text-align: center;
  font-size: 13px;
  color: rgba(255, 255, 255, 0.4);
}

.pl-list {
  flex: 1;
  overflow-y: auto;
  padding: 8px 0;
}

.pl-item {
  display: flex;
  align-items: center;
  gap: 12px;
  width: 100%;
  padding: 8px 20px;
  background: none;
  border: none;
  color: inherit;
  font-family: inherit;
  cursor: pointer;
  transition: background 0.12s;
  text-align: left;
}

.pl-item:hover { background: rgba(255, 255, 255, 0.04); }

.pl-item-cover {
  width: 48px;
  height: 48px;
  border-radius: 8px;
  background: linear-gradient(135deg, rgba(0, 245, 212, 0.2), rgba(36, 66, 255, 0.2));
  background-size: cover;
  background-position: center;
  flex-shrink: 0;
}

.pl-item-info { min-width: 0; flex: 1; }

.pl-item-name {
  font-size: 13px;
  font-weight: 600;
  color: rgba(255, 255, 255, 0.88);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.pl-item-meta {
  font-size: 11px;
  color: rgba(255, 255, 255, 0.4);
  margin-top: 3px;
}

.pl-tracks {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.pl-tracks-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 20px;
  font-size: 12px;
  color: rgba(255, 255, 255, 0.5);
}

.pl-play-all {
  padding: 5px 14px;
  border-radius: 8px;
  border: 1px solid rgba(0, 245, 212, 0.25);
  background: rgba(0, 245, 212, 0.08);
  color: rgba(0, 245, 212, 0.9);
  font-family: inherit;
  font-size: 11px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.15s;
}

.pl-play-all:hover { background: rgba(0, 245, 212, 0.16); }

.pl-track-list {
  flex: 1;
  overflow-y: auto;
  padding-bottom: 12px;
}

.pl-track {
  display: flex;
  align-items: center;
  gap: 10px;
  width: 100%;
  padding: 7px 20px;
  background: none;
  border: none;
  color: inherit;
  font-family: inherit;
  cursor: pointer;
  transition: background 0.12s;
  text-align: left;
}

.pl-track:hover { background: rgba(255, 255, 255, 0.04); }

.pl-track-idx {
  width: 24px;
  font-size: 11px;
  color: rgba(255, 255, 255, 0.3);
  text-align: right;
  flex-shrink: 0;
}

.pl-track-info { min-width: 0; flex: 1; }

.pl-track-name {
  font-size: 12.5px;
  color: rgba(255, 255, 255, 0.85);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.pl-track-artist {
  font-size: 11px;
  color: rgba(255, 255, 255, 0.38);
  margin-top: 1px;
}
</style>
