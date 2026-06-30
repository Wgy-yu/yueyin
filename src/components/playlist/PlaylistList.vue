<script setup lang="ts">
import type { PlaylistInfo } from "../../services/music";

defineProps<{
  playlists: PlaylistInfo[];
  loggedIn: boolean;
  error?: string;
}>();
defineEmits<{ (e: "open", id: string): void }>();
</script>

<template>
  <div class="shelf-list">
    <div v-if="error" class="shelf-empty text-red-300/80">
      {{ error }}
    </div>
    <div v-else-if="!playlists.length" class="shelf-empty">
      {{ loggedIn ? "暂无歌单" : "请先登录" }}
    </div>
    <template v-else>
      <button
        v-for="(pl, index) in playlists"
        :key="pl.id"
        class="shelf-card"
        :style="{ '--i': index }"
        @click="$emit('open', pl.id)"
      >
        <div class="shelf-cover" :style="pl.cover ? { backgroundImage: `url(${pl.cover})` } : undefined"></div>
        <div class="shelf-meta">
          <div class="shelf-label">我的歌单</div>
          <div class="shelf-title">{{ pl.name }}</div>
          <div class="shelf-sub">NE · {{ pl.trackCount }} 首 · 播放 {{ pl.playCount || 0 }}</div>
          <div class="shelf-line"></div>
          <div class="shelf-actions">
            <span>▶ 播放歌单</span>
            <span>详情</span>
          </div>
        </div>
      </button>
    </template>
  </div>
</template>

<style scoped>
.shelf-list {
  position: relative;
  flex: 1;
  min-height: 0;
  overflow-y: auto;
  overflow-x: visible;
  padding: 18px 26px 36px 10px;
  perspective: 1100px;
  scrollbar-width: thin;
  scrollbar-color: rgba(0, 245, 212, 0.28) transparent;
}

.shelf-list::-webkit-scrollbar {
  width: 4px;
}

.shelf-list::-webkit-scrollbar-thumb {
  border-radius: 999px;
  background: rgba(0, 245, 212, 0.24);
}

.shelf-empty {
  padding: 72px 20px;
  text-align: center;
  font-size: 13px;
  color: rgba(255, 255, 255, 0.42);
}

.shelf-card {
  --offset: min(var(--i), 10);
  position: relative;
  display: grid;
  grid-template-columns: 158px minmax(0, 1fr);
  gap: 18px;
  width: min(520px, 100%);
  min-height: 168px;
  margin: 0 0 -34px auto;
  padding: 16px 18px 16px 16px;
  border: 1px solid rgba(255, 255, 255, 0.18);
  border-radius: 24px;
  color: #fff;
  text-align: left;
  background:
    radial-gradient(circle at 18% 16%, rgba(255, 255, 255, 0.12), transparent 36%),
    linear-gradient(125deg, rgba(34, 36, 41, 0.66), rgba(4, 5, 8, 0.74) 62%, rgba(0, 0, 0, 0.86));
  box-shadow:
    0 24px 80px rgba(0, 0, 0, 0.42),
    inset 0 1px 0 rgba(255, 255, 255, 0.16),
    0 0 0 1px rgba(255, 255, 255, 0.035);
  backdrop-filter: blur(28px) saturate(1.26);
  -webkit-backdrop-filter: blur(28px) saturate(1.26);
  cursor: pointer;
  transform: translate3d(calc(var(--offset) * 5px), 0, calc(var(--offset) * -18px)) rotateY(-9deg) scale(calc(1 - var(--offset) * 0.008));
  transform-origin: right center;
  opacity: calc(1 - var(--offset) * 0.045);
  transition: transform 0.55s cubic-bezier(0.16, 1, 0.3, 1), opacity 0.35s, border-color 0.35s, box-shadow 0.35s;
  will-change: transform, opacity;
}

.shelf-card:hover {
  z-index: 5;
  border-color: rgba(255, 255, 255, 0.42);
  box-shadow:
    0 30px 94px rgba(0, 0, 0, 0.54),
    0 0 34px rgba(0, 245, 212, 0.12),
    inset 0 1px 0 rgba(255, 255, 255, 0.26);
  transform: translate3d(-20px, -6px, 40px) rotateY(-3deg) scale(1.035);
  opacity: 1;
}

.shelf-cover {
  position: relative;
  width: 158px;
  height: 136px;
  border-radius: 18px;
  background:
    radial-gradient(circle at 46% 44%, rgba(255, 255, 255, 0.88) 0 8%, rgba(0, 245, 212, 0.54) 9% 19%, rgba(36, 66, 255, 0.68) 20% 44%, rgba(12, 13, 18, 0.72) 45%),
    linear-gradient(135deg, rgba(0, 245, 212, 0.55), rgba(36, 66, 255, 0.6));
  background-size: cover;
  background-position: center;
  box-shadow: 0 18px 48px rgba(0, 0, 0, 0.34), inset 0 1px 0 rgba(255, 255, 255, 0.22);
  overflow: hidden;
}

.shelf-cover::after,
.shelf-card::after {
  content: "";
  position: absolute;
  inset: 0;
  pointer-events: none;
}

.shelf-cover::after {
  background: linear-gradient(145deg, rgba(255, 255, 255, 0.2), transparent 44%, rgba(0, 0, 0, 0.3));
}

.shelf-card::after {
  border-radius: inherit;
  background: linear-gradient(100deg, transparent, rgba(255, 255, 255, 0.08), transparent 58%);
  opacity: 0;
  transform: translateX(-45%);
  transition: opacity 0.32s, transform 0.55s cubic-bezier(0.16, 1, 0.3, 1);
}

.shelf-card:hover::after {
  opacity: 1;
  transform: translateX(38%);
}

.shelf-meta {
  position: relative;
  z-index: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.shelf-label {
  margin-bottom: 8px;
  font-size: 10px;
  font-weight: 800;
  letter-spacing: 0.12em;
  color: rgba(255, 255, 255, 0.46);
}

.shelf-title {
  font-size: 22px;
  line-height: 1.1;
  font-weight: 820;
  color: rgba(255, 255, 255, 0.94);
  overflow: hidden;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
}

.shelf-sub {
  margin-top: 16px;
  font-size: 11px;
  font-weight: 720;
  letter-spacing: 0.04em;
  color: rgba(255, 255, 255, 0.42);
}

.shelf-line {
  width: 100%;
  height: 1px;
  margin-top: 16px;
  background: linear-gradient(90deg, rgba(255, 255, 255, 0.14), rgba(255, 255, 255, 0.02));
}

.shelf-actions {
  display: flex;
  gap: 10px;
  margin-top: 12px;
  font-size: 11px;
  font-weight: 760;
  color: rgba(255, 255, 255, 0.78);
}

.shelf-actions span {
  height: 25px;
  padding: 0 13px;
  display: inline-flex;
  align-items: center;
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.07);
}

@media (max-width: 720px) {
  .shelf-card {
    grid-template-columns: 112px minmax(0, 1fr);
    min-height: 132px;
  }
  .shelf-cover {
    width: 112px;
    height: 100px;
  }
}
</style>
