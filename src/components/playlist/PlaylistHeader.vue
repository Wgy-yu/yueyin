<script setup lang="ts">
defineProps<{ hasTracks: boolean }>();
defineEmits<{ (e: "back"): void; (e: "close"): void }>();
</script>

<template>
  <div class="flex items-center gap-2.5 border-b border-white/[0.06] px-5 py-[18px]">
    <button
      v-if="hasTracks"
      class="rounded-md px-2 py-1 text-[13px] text-[rgba(0,245,212,.8)] transition-colors hover:bg-[rgba(0,245,212,.08)]"
      @click="$emit('back')"
    >
      ← 返回
    </button>
    <div class="flex-1 text-[15px] font-bold text-white/90">
      {{ hasTracks ? "歌单详情" : "我的歌单" }}
    </div>
    <button class="rounded-md px-2 py-1 text-xl text-white/50 transition-colors hover:bg-white/[0.08] hover:text-white" @click="$emit('close')">×</button>
  </div>
</template>
