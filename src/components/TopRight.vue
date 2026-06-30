<script setup lang="ts">
import { computed, ref, onMounted } from "vue";
import { useRouter } from "vue-router";
import { useAccountStore } from "../stores/account";
import LoginModal from "./LoginModal.vue";

const router = useRouter();
const account = useAccountStore();
const showLogin = ref(false);
const current = computed(() => account.current());

function goHome() {
  router.push("/");
}

onMounted(() => {
  account.refreshStatus();
});
</script>

<template>
  <div class="fixed right-6 top-6 z-10 flex items-center gap-2.5">
    <!-- Home button -->
    <button
      class="flex h-11 w-11 items-center justify-center rounded-full border border-[rgba(0,245,212,.18)] bg-white/[0.04] text-[rgba(232,236,239,.78)] backdrop-blur-xl transition-all hover:border-[rgba(0,245,212,.44)] hover:bg-[rgba(0,245,212,.075)] hover:text-white hover:shadow-[0_14px_38px_rgba(0,245,212,.08)]"
      @click="goHome"
      title="回到首页"
      aria-label="回到首页"
    >
      <svg width="19" height="19" fill="none" stroke="currentColor" stroke-width="1.9" viewBox="0 0 24 24">
        <path d="M3 10.8 12 3l9 7.8" />
        <path d="M5 10v10h14V10" />
        <path d="M9.5 20v-5h5v5" />
      </svg>
    </button>

    <!-- User button -->
    <button
      class="flex h-11 items-center justify-center gap-2 rounded-[22px] border border-white/[0.08] bg-white/[0.035] px-4 text-[12px] tracking-wide text-white/55 backdrop-blur-xl transition-all hover:bg-white/[0.09] hover:text-white"
      @click="showLogin = true"
      :title="current.loggedIn ? current.nickname || current.userId || '已登录' : '登录账号'"
    >
      <img
        v-if="current.loggedIn && current.avatar"
        :src="current.avatar"
        class="h-7 w-7 rounded-full object-cover"
        alt=""
      />
      <span v-else-if="current.loggedIn" class="max-w-24 truncate font-semibold">
        {{ current.nickname || current.userId || '已登录' }}
      </span>
      <span v-else class="font-semibold">登录</span>
    </button>

    <LoginModal v-if="showLogin" @close="showLogin = false" />
  </div>
</template>
