<script setup lang="ts">
import { ref } from "vue";
import { useAccountStore } from "../stores/account";
import PlaylistPanel from "./PlaylistPanel.vue";

const account = useAccountStore();
const showPlaylist = ref(false);

const cards = ref([
  {
    id: 1,
    label: "Library",
    title: "我的歌单",
    sub: "打开左侧歌单库",
    tone: "library",
  },
  {
    id: 2,
    label: "Daily",
    title: "每日推荐",
    sub: account.current().loggedIn ? "登录后同步你的今日歌曲" : "请先登录",
    tone: "mix",
  },
  {
    id: 3,
    label: "Song",
    title: "私人电台",
    sub: "从你的推荐和歌单里开播",
    tone: "playlist",
  },
  {
    id: 4,
    label: "Continue",
    title: "继续听",
    sub: "最近播放会出现在这里",
    tone: "mix",
  },
  {
    id: 5,
    label: "Profile",
    title: "听歌画像",
    sub: "播放几首后生成偏好",
    tone: "local",
  },
  {
    id: 6,
    label: "Song",
    title: "常听歌手",
    sub: "你的偏好会在这里汇总",
    tone: "local",
  },
]);

const tiles = ref([
  { id: 1, title: "天气电台", sub: "根据天气推荐音乐", tone: "search" },
  { id: 2, title: "热门播客", sub: "发现有趣的播客", tone: "podcast" },
  { id: 3, title: "新歌速递", sub: "最新发布的歌曲", tone: "playlist" },
  { id: 4, title: "私人雷达", sub: "为你精选的歌曲", tone: "guide" },
  { id: 5, title: "排行榜", sub: "热门歌曲排行", tone: "library" },
]);
</script>

<template>
  <section id="empty-home" class="active">
    <div class="empty-home-shell">
      <div class="home-hero">
        <div class="home-hero-inner">
          <div class="home-kicker">YUEYIN · 悦音</div>
          <div class="home-title">沉浸式<br />音乐空间</div>
          <div class="home-sub">
            搜索或导入一首歌即可播放；登录后会同步歌单、红心与播客。
          </div>
          <div class="home-quick-row">
            <button class="home-chip">搜索歌曲</button>
            <button class="home-chip">导入音乐</button>
          </div>
        </div>
      </div>

      <div class="home-grid">
        <button
          v-for="card in cards"
          :key="card.id"
          class="home-card"
          :data-home-tone="card.tone"
          @click="card.id === 1 ? showPlaylist = true : null"
        >
          <div class="home-card-label">{{ card.label }}</div>
          <div class="home-card-title">{{ card.title }}</div>
          <div class="home-card-sub">{{ card.sub }}</div>
          <div class="home-card-art"></div>
        </button>
      </div>

      <div class="home-rail">
        <div class="home-section-head">
          <div class="home-section-title">为你准备</div>
          <div class="home-section-note">正在整理推荐</div>
        </div>
        <div class="home-tile-row">
          <button
            v-for="tile in tiles"
            :key="tile.id"
            class="home-tile"
            :data-home-tone="tile.tone"
          >
            <div class="home-tile-cover"></div>
            <div class="home-tile-title">{{ tile.title }}</div>
            <div class="home-tile-sub">{{ tile.sub }}</div>
          </button>
        </div>
      </div>
    </div>
    </section>
    <PlaylistPanel v-if="showPlaylist" @close="showPlaylist = false" />
  </template>

<style scoped>
#empty-home {
  position: fixed;
  z-index: 4;
  left: 50%;
  top: 158px;
  bottom: 58px;
  width: min(1240px, calc(100vw - 72px));
  transform: translateX(-50%) translateY(0) scale(1);
  opacity: 1;
  pointer-events: auto;
  transition: opacity 0.72s cubic-bezier(0.16, 1, 0.3, 1),
    transform 0.72s cubic-bezier(0.16, 1, 0.3, 1);
}

.empty-home-shell {
  height: 100%;
  min-height: 440px;
  display: grid;
  grid-template-columns: minmax(370px, 0.94fr) minmax(520px, 1.06fr);
  grid-template-rows: auto 1fr;
  gap: 14px;
  align-content: stretch;
}

.home-hero {
  position: relative;
  grid-row: 1 / span 2;
  min-height: 438px;
  padding: 28px;
  border-radius: 28px;
  border: 1px solid rgba(0, 245, 212, 0.22);
  background: linear-gradient(
    145deg,
    rgba(33, 29, 34, 0.64),
    rgba(9, 10, 14, 0.76) 48%,
    rgba(17, 20, 25, 0.7)
  );
  box-shadow: 0 28px 90px rgba(0, 0, 0, 0.34),
    0 0 0 1px rgba(0, 245, 212, 0.07),
    inset 0 1px 0 rgba(255, 255, 255, 0.065);
  backdrop-filter: blur(30px) saturate(1.16);
  -webkit-backdrop-filter: blur(30px) saturate(1.16);
  overflow: hidden;
}

.home-hero::before {
  content: "";
  position: absolute;
  inset: 0;
  background: linear-gradient(
      118deg,
      rgba(255, 255, 255, 0.06),
      transparent 24%,
      rgba(0, 245, 212, 0.07) 54%,
      rgba(36, 66, 255, 0.055) 82%,
      transparent 100%
    ),
    linear-gradient(
      90deg,
      rgba(255, 255, 255, 0.032) 0 1px,
      transparent 1px 48px
    ),
    linear-gradient(
      0deg,
      rgba(255, 255, 255, 0.024) 0 1px,
      transparent 1px 44px
    );
  opacity: 0.72;
  pointer-events: none;
}

.home-hero::after {
  content: "";
  position: absolute;
  left: 28px;
  right: 28px;
  bottom: 22px;
  height: 1px;
  background: linear-gradient(
    90deg,
    transparent,
    rgba(0, 245, 212, 0.52),
    rgba(36, 66, 255, 0.24),
    rgba(248, 244, 238, 0.18),
    transparent
  );
  opacity: 0.82;
  pointer-events: none;
}

.home-hero-inner {
  position: relative;
  z-index: 2;
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.home-kicker {
  font-size: 10px;
  font-weight: 780;
  letter-spacing: 0.18em;
  color: rgba(0, 245, 212, 0.62);
  text-transform: uppercase;
  margin-bottom: 14px;
}

.home-title {
  font-size: clamp(34px, 4.6vw, 58px);
  line-height: 0.98;
  font-weight: 760;
  letter-spacing: 0;
  color: rgba(255, 255, 255, 0.98);
  max-width: 380px;
  text-shadow: 0 10px 36px rgba(0, 0, 0, 0.26);
}

.home-sub {
  margin-top: 14px;
  font-size: 13px;
  line-height: 1.62;
  color: rgba(255, 255, 255, 0.62);
  max-width: 348px;
}

.home-quick-row {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-top: 22px;
}

.home-chip {
  height: 32px;
  padding: 0 13px;
  border-radius: 999px;
  border: 1px solid rgba(255, 255, 255, 0.11);
  background: rgba(255, 255, 255, 0.06);
  color: rgba(255, 255, 255, 0.76);
  font-family: inherit;
  font-size: 11.5px;
  font-weight: 680;
  cursor: pointer;
  transition: background 0.18s, border-color 0.18s, color 0.18s, transform 0.18s,
    box-shadow 0.18s;
}

.home-chip:hover {
  background: rgba(0, 245, 212, 0.1);
  border-color: rgba(0, 245, 212, 0.38);
  color: #fff;
  box-shadow: 0 10px 26px rgba(0, 245, 212, 0.08);
  transform: translateY(-1px);
}

.home-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 12px;
}

.home-card {
  --tone-a: #00f5d4;
  --tone-b: #2442ff;
  --tone-c: #f8f4ee;
  position: relative;
  min-height: 152px;
  border: 1px solid rgba(0, 245, 212, 0.1);
  border-radius: 22px;
  background: linear-gradient(
    142deg,
    rgba(18, 21, 26, 0.66),
    rgba(8, 9, 13, 0.76)
  );
  box-shadow: 0 20px 64px rgba(0, 0, 0, 0.28),
    inset 0 1px 0 rgba(255, 255, 255, 0.06);
  backdrop-filter: blur(24px) saturate(1.12);
  -webkit-backdrop-filter: blur(24px) saturate(1.12);
  padding: 17px;
  overflow: hidden;
  color: #fff;
  text-align: left;
  font-family: inherit;
  cursor: pointer;
  transition: transform 0.22s cubic-bezier(0.16, 1, 0.3, 1),
    border-color 0.22s, background 0.22s, box-shadow 0.22s;
}

.home-card[data-home-tone="mix"] {
  --tone-a: #9db8cf;
  --tone-b: #00f5d4;
  --tone-c: #2442ff;
}

.home-card[data-home-tone="local"] {
  --tone-a: #f8f4ee;
  --tone-b: #00f5d4;
  --tone-c: #2442ff;
}

.home-card[data-home-tone="library"] {
  --tone-a: #00f5d4;
  --tone-b: #f8f4ee;
  --tone-c: #2442ff;
}

.home-card::before {
  content: "";
  position: absolute;
  inset: 0;
  background: linear-gradient(
      118deg,
      rgba(var(--tone-a), 0.22),
      transparent 38%,
      rgba(var(--tone-b), 0.16) 74%,
      transparent
    ),
    linear-gradient(
      90deg,
      rgba(255, 255, 255, 0.035) 0 1px,
      transparent 1px 38px
    );
  opacity: 0.86;
  pointer-events: none;
}

.home-card:hover {
  transform: translateY(-3px);
  border-color: rgba(0, 245, 212, 0.42);
  background: linear-gradient(
    142deg,
    rgba(36, 33, 39, 0.72),
    rgba(10, 10, 14, 0.84)
  );
  box-shadow: 0 28px 84px rgba(0, 0, 0, 0.36),
    0 0 34px rgba(0, 245, 212, 0.16), inset 0 1px 0 rgba(255, 255, 255, 0.085);
}

.home-card-label {
  font-size: 10px;
  font-weight: 760;
  letter-spacing: 0.14em;
  text-transform: uppercase;
  color: rgba(0, 245, 212, 0.7);
  margin-bottom: 8px;
  text-shadow: 0 0 18px rgba(0, 245, 212, 0.22);
}

.home-card-title {
  font-size: 19px;
  font-weight: 780;
  line-height: 1.16;
  letter-spacing: 0;
  color: rgba(255, 255, 255, 0.96);
  max-width: 70%;
}

.home-card-sub {
  margin-top: 8px;
  font-size: 11.5px;
  line-height: 1.45;
  color: rgba(255, 255, 255, 0.55);
  max-width: 70%;
}

.home-card-art {
  position: absolute;
  z-index: 1;
  right: 13px;
  bottom: 13px;
  width: 108px;
  height: 108px;
  border-radius: 24px;
  background: linear-gradient(135deg, #00f5d4, #2442ff);
  box-shadow: 0 18px 50px rgba(0, 0, 0, 0.36),
    inset 0 1px 0 rgba(255, 255, 255, 0.16);
  transform: rotate(3deg);
  overflow: hidden;
}

.home-card-art::before {
  content: "";
  position: absolute;
  inset: 18px;
  border-radius: 50%;
  background: repeating-radial-gradient(
      circle,
      rgba(255, 255, 255, 0.16) 0 1px,
      transparent 1px 8px
    ),
    conic-gradient(from 180deg, #00f5d4, #2442ff, #f8f4ee, #00f5d4);
  box-shadow: inset 0 0 0 14px rgba(5, 5, 8, 0.44),
    0 12px 28px rgba(0, 0, 0, 0.26);
}

.home-card-art::after {
  content: "";
  position: absolute;
  inset: 0;
  background: linear-gradient(
    145deg,
    rgba(255, 255, 255, 0.12),
    transparent 42%,
    rgba(0, 0, 0, 0.28)
  );
  pointer-events: none;
}

.home-rail {
  align-self: stretch;
  min-height: 0;
  display: grid;
  grid-template-rows: auto 1fr;
  gap: 12px;
}

.home-section-head {
  display: flex;
  align-items: end;
  justify-content: space-between;
  gap: 12px;
  min-height: 38px;
}

.home-section-title {
  font-size: 13px;
  font-weight: 760;
  color: rgba(255, 255, 255, 0.84);
  letter-spacing: 0.04em;
}

.home-section-note {
  font-size: 11px;
  color: rgba(255, 255, 255, 0.38);
  white-space: nowrap;
}

.home-tile-row {
  display: grid;
  grid-template-columns: repeat(5, minmax(0, 1fr));
  gap: 10px;
  min-height: 0;
}

.home-tile {
  --tone-a: #00f5d4;
  --tone-b: #2442ff;
  --tone-c: #f8f4ee;
  position: relative;
  min-width: 0;
  min-height: 166px;
  border: 1px solid rgba(0, 245, 212, 0.09);
  border-radius: 20px;
  background: linear-gradient(
    145deg,
    rgba(255, 255, 255, 0.06),
    rgba(255, 255, 255, 0.025)
  );
  box-shadow: 0 16px 50px rgba(0, 0, 0, 0.22),
    inset 0 1px 0 rgba(255, 255, 255, 0.052);
  backdrop-filter: blur(20px) saturate(1.1);
  -webkit-backdrop-filter: blur(20px) saturate(1.1);
  padding: 10px;
  color: #fff;
  text-align: left;
  font-family: inherit;
  cursor: pointer;
  transition: transform 0.2s, border-color 0.2s, background 0.2s,
    box-shadow 0.2s;
  overflow: hidden;
}

.home-tile[data-home-tone="search"],
.home-tile[data-home-tone="playlist"] {
  --tone-a: #9db8cf;
  --tone-b: #00f5d4;
  --tone-c: #2442ff;
}

.home-tile[data-home-tone="local"],
.home-tile[data-home-tone="podcast"] {
  --tone-a: #f8f4ee;
  --tone-b: #00f5d4;
  --tone-c: #2442ff;
}

.home-tile[data-home-tone="guide"],
.home-tile[data-home-tone="library"] {
  --tone-a: #00f5d4;
  --tone-b: #f8f4ee;
  --tone-c: #2442ff;
}

.home-tile::before {
  content: "";
  position: absolute;
  left: 10px;
  right: 10px;
  top: 10px;
  height: 92px;
  border-radius: 15px;
  background: linear-gradient(
    135deg,
    rgba(0, 245, 212, 0.12),
    transparent 48%,
    rgba(36, 66, 255, 0.1)
  );
  opacity: 0.9;
  pointer-events: none;
}

.home-tile:hover {
  transform: translateY(-3px);
  border-color: rgba(0, 245, 212, 0.36);
  background: rgba(255, 255, 255, 0.068);
  box-shadow: 0 22px 62px rgba(0, 0, 0, 0.28),
    0 0 26px rgba(0, 245, 212, 0.12);
}

.home-tile-cover {
  position: relative;
  width: 100%;
  height: 92px;
  border-radius: 15px;
  background: linear-gradient(135deg, #00f5d4, #2442ff);
  box-shadow: 0 14px 34px rgba(0, 0, 0, 0.26),
    inset 0 1px 0 rgba(255, 255, 255, 0.14);
  margin-bottom: 10px;
  overflow: hidden;
}

.home-tile-cover::before {
  content: "";
  position: absolute;
  left: 12px;
  bottom: 12px;
  width: 54px;
  height: 54px;
  border-radius: 50%;
  background: repeating-radial-gradient(
      circle,
      rgba(255, 255, 255, 0.14) 0 1px,
      transparent 1px 7px
    ),
    conic-gradient(from 180deg, #00f5d4, #2442ff, #f8f4ee, #00f5d4);
  box-shadow: inset 0 0 0 10px rgba(5, 5, 8, 0.42);
}

.home-tile-title {
  font-size: 12px;
  font-weight: 720;
  color: rgba(255, 255, 255, 0.9);
  line-height: 1.28;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.home-tile-sub {
  font-size: 10.5px;
  color: rgba(255, 255, 255, 0.42);
  margin-top: 5px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

@media (max-width: 1120px) {
  #empty-home {
    width: calc(100vw - 48px);
  }

  .empty-home-shell {
    grid-template-columns: minmax(300px, 0.88fr) minmax(440px, 1.12fr);
    gap: 12px;
  }

  .home-hero {
    padding: 24px;
  }

  .home-title {
    font-size: clamp(30px, 4vw, 48px);
    max-width: 340px;
  }

  .home-sub {
    font-size: 12.5px;
    line-height: 1.5;
    max-width: 314px;
  }

  .home-card {
    min-height: 126px;
    padding: 14px;
    border-radius: 20px;
  }

  .home-card-title {
    font-size: 16.5px;
    max-width: 66%;
  }

  .home-card-art {
    right: 11px;
    bottom: 11px;
    width: 80px;
    height: 80px;
    border-radius: 18px;
  }
}

@media (max-width: 760px) {
  #empty-home {
    top: 150px;
    bottom: 56px;
    width: calc(100vw - 40px);
  }

  .empty-home-shell {
    grid-template-columns: 1fr;
    grid-template-rows: auto auto 1fr;
    overflow: auto;
    padding-bottom: 4px;
  }

  .home-hero {
    grid-row: auto;
    min-height: 344px;
  }

  .home-grid {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }

  .home-tile-row {
    grid-template-columns: repeat(3, minmax(0, 1fr));
  }
}

@media (max-width: 620px) {
  #empty-home {
    top: 146px;
    bottom: 44px;
    width: calc(100vw - 24px);
  }

  .home-hero {
    padding: 20px;
    border-radius: 22px;
  }

  .home-grid {
    grid-template-columns: 1fr;
  }

  .home-tile-row {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
}
</style>
