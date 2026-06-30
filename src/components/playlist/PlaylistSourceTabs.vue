<script setup lang="ts">
import type { SourceType } from "../../types/track";

defineProps<{ activeSource: SourceType }>();
defineEmits<{ (e: "switch", source: SourceType): void }>();

const sources: { id: SourceType; label: string }[] = [
  { id: "netease", label: "网易云" },
  { id: "qq", label: "QQ" },
];
</script>

<template>
  <div class="flex gap-1 px-5 py-2.5">
    <button
      v-for="source in sources"
      :key="source.id"
      class="h-8 flex-1 rounded-lg border text-[12px] font-semibold transition-all"
      :class="activeSource === source.id ? 'border-[rgba(0,245,212,.3)] bg-[rgba(0,245,212,.12)] text-[rgba(0,245,212,.9)]' : 'border-white/[0.08] bg-white/[0.04] text-white/60'"
      @click="$emit('switch', source.id)"
    >
      {{ source.label }}
    </button>
  </div>
</template>
