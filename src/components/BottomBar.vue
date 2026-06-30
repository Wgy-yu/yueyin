<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from "vue";
import { storeToRefs } from "pinia";
import { bindButtonAnimations, unbindButtonAnimations } from "../utils/animations";
import { usePlayerStore } from "../stores/player";
import { useQueueStore } from "../stores/queue";
import { animateModeSwitch } from "../utils/animations";

const barRef = ref<HTMLElement | null>(null);
const isLiked = ref(false);
const player = usePlayerStore();
const queue = useQueueStore();
const { playing: isPlaying, currentTime, duration } = storeToRefs(player);
const progress = computed(() =>
  duration.value > 0 ? (currentTime.value / duration.value) * 100 : 0
);

const playModeLabel = computed(() => {
  const map = { loop: "顺序", shuffle: "随机", single: "单曲" };
  return map[player.playMode];
});

onMounted(() => {
  if (barRef.value) bindButtonAnimations(barRef.value);
});
onUnmounted(() => {
  if (barRef.value) unbindButtonAnimations(barRef.value);
});

function togglePlay() {
  isPlaying.value = !isPlaying.value;
}

function toggleLike() {
  isLiked.value = !isLiked.value;
}

function handleNext() {
  queue.currentIndex = queue.nextIndex(player.playMode);
}

function handlePrev() {
  queue.currentIndex = queue.prevIndex();
}

function handleCycleMode(e: Event) {
  player.cyclePlayMode();
  const btn = (e.currentTarget as HTMLElement).closest(".ctrl-btn") as HTMLElement;
  if (btn) animateModeSwitch(btn);
}

function formatTime(seconds: number): string {
  const min = Math.floor(seconds / 60);
  const sec = Math.floor(seconds % 60);
  return `${min}:${sec.toString().padStart(2, "0")}`;
}
</script>

<template>
  <div id="bottom-bar" ref="barRef" class="visible">
    <div id="progress-bar">
      <div id="progress-fill" :style="{ width: progress + '%' }"></div>
      <div
        id="progress-thumb"
        :style="{ left: progress + '%' }"
        aria-hidden="true"
      ></div>
    </div>
    <div id="controls">
      <div class="control-cluster actions">
        <div class="control-track">
          <div id="control-cover" class="control-cover cover-empty"></div>
          <div class="control-meta">
            <div id="control-title" class="control-title">{{ player.currentTrack?.name ?? '未播放' }}</div>
            <div id="control-artist" class="control-artist">{{ player.currentTrack?.artist ?? '选择一首歌曲' }}</div>
          </div>
        </div>
        <button
          id="heart-btn"
          class="ctrl-btn"
          :class="{ liked: isLiked }"
          @click="toggleLike"
          title="红心喜欢"
        >
          <svg viewBox="0 0 24 24" aria-hidden="true">
            <path
              d="M12 21.45c-.32 0-.62-.12-.86-.34l-1.23-1.12C5.54 16.03 2.25 13.05 2.25 8.9 2.25 5.48 4.88 2.9 8.28 2.9c1.7 0 3.35.72 4.52 1.96C13.97 3.62 15.62 2.9 17.32 2.9c3.4 0 6.03 2.58 6.03 6 0 4.15-3.29 7.13-7.66 11.09l-1.23 1.12c-.24.22-.54.34-.86.34z"
            />
          </svg>
        </button>
      </div>

      <div class="control-cluster transport">
        <button class="ctrl-btn" :title="playModeLabel" @click="handleCycleMode">
          <svg
            width="19"
            height="19"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            viewBox="0 0 24 24"
          >
            <path d="M17 2l4 4-4 4" />
            <path d="M3 11V9a4 4 0 0 1 4-4h14" />
            <path d="M7 22l-4-4 4-4" />
            <path d="M21 13v2a4 4 0 0 1-4 4H3" />
          </svg>
        </button>
        <button class="ctrl-btn" title="上一首" @click="handlePrev">
          <svg width="18" height="18" fill="currentColor" viewBox="0 0 24 24">
            <path d="M6 6h2v12H6zm3.5 6l8.5 6V6z" />
          </svg>
        </button>
        <button
          id="play-btn"
          class="ctrl-btn"
          @click="togglePlay"
          title="播放/暂停"
        >
          <svg
            v-if="!isPlaying"
            width="20"
            height="20"
            fill="currentColor"
            viewBox="0 0 24 24"
          >
            <path d="M8 5v14l11-7z" />
          </svg>
          <svg
            v-else
            width="20"
            height="20"
            fill="currentColor"
            viewBox="0 0 24 24"
          >
            <path d="M6 4h4v16H6zM14 4h4v16h-4z" />
          </svg>
        </button>
        <button class="ctrl-btn" title="下一首" @click="handleNext">
          <svg width="18" height="18" fill="currentColor" viewBox="0 0 24 24">
            <path d="M6 18l8.5-6L6 6v12zM16 6v12h2V6h-2z" />
          </svg>
        </button>
        <button class="ctrl-btn" title="当前队列">
          <svg
            width="19"
            height="19"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            viewBox="0 0 24 24"
          >
            <path d="M8 6h13" />
            <path d="M8 12h13" />
            <path d="M8 18h13" />
            <path d="M3 6h.01" />
            <path d="M3 12h.01" />
            <path d="M3 18h.01" />
          </svg>
        </button>
      </div>

      <div class="control-cluster modes">
        <button class="ctrl-btn lyrics-toggle-btn" title="歌词">
          <span class="lyrics-word-icon">词</span>
        </button>
        <div class="volume-control">
          <button class="ctrl-btn" title="音量 / 静音" @click="player.toggleMute()">
            <svg
              width="18"
              height="18"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              viewBox="0 0 24 24"
            >
              <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" />
              <path d="M15 9.5a4 4 0 0 1 0 5" />
            </svg>
          </button>
        </div>
        <div id="time-display">
          {{ formatTime(currentTime) }} / {{ formatTime(duration) }}
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
#bottom-bar {
  position: fixed;
  z-index: 6;
  bottom: 16px;
  left: 50%;
  transform: translateX(-50%) translateY(0) scale(1);
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
  opacity: 0.91;
  pointer-events: auto;
  width: min(1080px, calc(100vw - 56px));
  padding: 9px 22px 14px;
  border-radius: 50px;
  background: rgba(0, 0, 0, 0.1);
  backdrop-filter: blur(12px) saturate(1.8) brightness(1.16);
  -webkit-backdrop-filter: blur(12px) saturate(1.8) brightness(1.16);
  box-shadow: inset 0 0 2px 1px rgba(255, 255, 255, 0.35),
    inset 0 0 10px 4px rgba(255, 255, 255, 0.15),
    0 4px 16px rgba(17, 17, 26, 0.05), 0 8px 24px rgba(17, 17, 26, 0.05),
    0 16px 56px rgba(17, 17, 26, 0.05),
    inset 0 4px 16px rgba(17, 17, 26, 0.05),
    inset 0 8px 24px rgba(17, 17, 26, 0.05),
    inset 0 16px 56px rgba(17, 17, 26, 0.05);
}

#progress-bar {
  position: relative;
  z-index: 1;
  align-self: center;
  width: calc(100% - clamp(86px, 10vw, 156px));
  height: 4px;
  margin: 4px auto 1px;
  background: rgba(255, 255, 255, 0.095);
  border-radius: 999px;
  cursor: pointer;
  transition: height 0.2s, background 0.2s, box-shadow 0.2s, width 0.2s;
  overflow: visible;
  box-shadow: inset 0 1px 1px rgba(255, 255, 255, 0.12),
    inset 0 -1px 1px rgba(0, 0, 0, 0.2);
}

#progress-bar:hover {
  height: 5px;
  background: rgba(255, 255, 255, 0.14);
  box-shadow: 0 0 18px rgba(0, 245, 212, 0.1),
    inset 0 1px 1px rgba(255, 255, 255, 0.18);
}

#progress-fill {
  height: 100%;
  background: linear-gradient(90deg, #00f5d4, #2442ff);
  border-radius: 999px;
  transition: width 0.1s linear;
}

#progress-thumb {
  position: absolute;
  top: 50%;
  left: 0;
  width: 12px;
  height: 12px;
  border-radius: 50%;
  background: #fff;
  box-shadow: 0 0 8px rgba(0, 245, 212, 0.4);
  transform: translate(-50%, -50%) scale(0);
  opacity: 0;
  transition: opacity 0.2s, transform 0.2s;
}

#progress-bar:hover #progress-thumb {
  opacity: 1;
  transform: translate(-50%, -50%) scale(1);
}

#controls {
  display: grid;
  grid-template-columns: minmax(310px, 1.05fr) minmax(344px, auto) minmax(230px, 0.72fr);
  align-items: center;
  width: 100%;
  gap: 8px;
}

.control-cluster {
  display: flex;
  align-items: center;
  gap: 6px;
}

.control-cluster.actions {
  justify-content: flex-start;
}

.control-cluster.transport {
  justify-content: center;
}

.control-cluster.modes {
  justify-content: flex-end;
}

.control-track {
  display: flex;
  align-items: center;
  gap: 12px;
  flex: 1;
  min-width: 0;
}

.control-cover {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  background: linear-gradient(135deg, rgba(0, 245, 212, 0.3), rgba(36, 66, 255, 0.3));
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3);
  flex-shrink: 0;
  overflow: hidden;
}

.control-cover.cover-empty::before {
  content: "";
  position: absolute;
  inset: 25%;
  border-radius: 50%;
  background: repeating-radial-gradient(
    circle,
    rgba(255, 255, 255, 0.14) 0 1px,
    transparent 1px 7px
  );
}

.control-meta {
  flex: 1;
  min-width: 0;
}

.control-title {
  font-size: 13px;
  font-weight: 500;
  color: rgba(255, 255, 255, 0.92);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  cursor: pointer;
}

.control-title:hover {
  color: #fff;
}

.control-artist {
  font-size: 11px;
  color: rgba(255, 255, 255, 0.45);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  cursor: pointer;
  margin-top: 2px;
}

.control-artist:hover {
  color: rgba(255, 255, 255, 0.7);
}

.ctrl-btn {
  flex: 0 0 auto;
  width: 36px;
  height: 36px;
  background: transparent;
  border: 0;
  border-radius: 11px;
  color: rgba(255, 255, 255, 0.7);
  cursor: pointer;
  /* ponytail: transform/box-shadow handled by GSAP; keep color/background only */
  transition: color 0.18s, background 0.18s;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0;
  will-change: transform;
}

.ctrl-btn svg {
  width: 21px;
  height: 21px;
}

.ctrl-btn:hover {
  color: #fff;
  background: rgba(255, 255, 255, 0.045);
  text-shadow: 0 0 10px rgba(0, 245, 212, 0.12);
}

.ctrl-btn.liked {
  color: #ff7a90;
  text-shadow: 0 0 18px rgba(255, 122, 144, 0.36);
}

#play-btn {
  width: 58px;
  height: 58px;
  border: 0;
  border-radius: 50%;
  color: rgba(255, 255, 255, 0.96);
  background: rgba(0, 0, 0, 0.1);
  /* ponytail: transform handled by GSAP */
  transition: background 0.2s, box-shadow 0.2s;
  box-shadow: inset 0 0 2px 1px rgba(255, 255, 255, 0.34),
    inset 0 0 10px 4px rgba(255, 255, 255, 0.13),
    0 10px 30px rgba(0, 0, 0, 0.18);
  backdrop-filter: blur(12px) saturate(1.8) brightness(1.16);
  -webkit-backdrop-filter: blur(12px) saturate(1.8) brightness(1.16);
}

#play-btn svg {
  width: 24px;
  height: 24px;
}

#play-btn:hover {
  background: rgba(255, 255, 255, 0.055);
  box-shadow: inset 0 0 2px 1px rgba(255, 255, 255, 0.42),
    inset 0 0 12px 5px rgba(255, 255, 255, 0.17),
    0 12px 34px rgba(0, 0, 0, 0.22), 0 0 18px rgba(0, 245, 212, 0.1);
}

#play-btn:active {
  box-shadow: inset 0 0 2px 1px rgba(255, 255, 255, 0.28),
    inset 0 0 10px 4px rgba(255, 255, 255, 0.1),
    0 8px 22px rgba(0, 0, 0, 0.2);
}

.lyrics-word-icon {
  font-size: 13px;
  font-weight: 700;
  letter-spacing: 0.5px;
}

.volume-control {
  position: relative;
}

#time-display {
  font-size: 11px;
  font-weight: 500;
  color: rgba(255, 255, 255, 0.45);
  letter-spacing: 0.5px;
  min-width: 80px;
  text-align: right;
}

@media (max-width: 920px) {
  #bottom-bar {
    width: calc(100vw - 28px);
    padding: 9px 14px 12px;
  }

  #controls {
    grid-template-columns: minmax(0, 1fr) auto minmax(0, 1fr);
    gap: 8px;
  }

  .control-cluster {
    gap: 8px;
    height: 56px;
  }

  .control-cover {
    width: 44px;
    height: 44px;
  }

  .control-meta {
    max-width: 170px;
  }

  .control-title {
    font-size: 12.5px;
  }

  .control-artist {
    font-size: 11px;
  }

  .ctrl-btn {
    width: 32px;
    height: 32px;
  }

  .ctrl-btn svg {
    width: 19px;
    height: 19px;
  }

  #play-btn {
    width: 54px;
    height: 54px;
  }

  #time-display {
    display: none;
  }
}

@media (max-width: 620px) {
  #bottom-bar {
    width: calc(100vw - 20px);
  }

  #controls {
    grid-template-columns: 1fr;
  }

  .control-cluster {
    grid-column: 1 !important;
    justify-content: center;
  }

  .control-cluster.transport {
    order: 1;
  }

  .control-cluster.actions {
    order: 2;
  }

  .control-cluster.modes {
    order: 3;
  }

  .control-track {
    display: none;
  }
}
</style>
