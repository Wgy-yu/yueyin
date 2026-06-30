<script setup lang="ts">
import type { HomeCard } from "./types";

defineProps<{ cards: HomeCard[] }>();
defineEmits<{ (e: "open-playlists"): void }>();
</script>

<template>
  <div class="home-grid">
    <button
      v-for="card in cards"
      :key="card.id"
      class="home-card"
      :data-home-tone="card.tone"
      @click="card.id === 1 && $emit('open-playlists')"
    >
      <div class="home-card-label">{{ card.label }}</div>
      <div class="home-card-title">{{ card.title }}</div>
      <div class="home-card-sub">{{ card.sub }}</div>
      <div class="home-card-art"></div>
    </button>
  </div>
</template>
