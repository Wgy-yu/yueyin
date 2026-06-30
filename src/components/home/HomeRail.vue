<script setup lang="ts">
import type { HomeTile } from "./types";

defineProps<{ tiles: HomeTile[] }>();
</script>

<template>
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
</template>
