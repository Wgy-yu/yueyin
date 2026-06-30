<script setup lang="ts">
import { useRouter } from "vue-router";

const router = useRouter();

function goHome() {
  router.push("/");
}
</script>

<template>
  <div id="top-right">
    <button
      id="home-btn"
      class="icon-btn"
      @click="goHome"
      title="回到首页"
      aria-label="回到首页"
    >
      <svg
        width="19"
        height="19"
        fill="none"
        stroke="currentColor"
        stroke-width="1.9"
        viewBox="0 0 24 24"
      >
        <path d="M3 10.8 12 3l9 7.8" />
        <path d="M5 10v10h14V10" />
        <path d="M9.5 20v-5h5v5" />
      </svg>
    </button>
    <button id="user-btn" class="icon-btn" title="登录账号">
      <span class="login-word">登录</span>
    </button>
  </div>
</template>

<style scoped>
#top-right {
  position: fixed;
  z-index: 10;
  top: 24px;
  right: 24px;
  display: flex;
  align-items: center;
  gap: 10px;
}

.icon-btn {
  width: 44px;
  height: 44px;
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.035);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  color: rgba(255, 255, 255, 0.55);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s;
  font-family: inherit;
  padding: 0;
}

.icon-btn:hover {
  background: rgba(255, 255, 255, 0.09);
  border-color: rgba(255, 255, 255, 0.18);
  color: #fff;
  transform: scale(1.04);
}

#home-btn {
  width: 44px;
  min-width: 44px;
  border-color: rgba(0, 245, 212, 0.18);
  background: rgba(255, 255, 255, 0.04);
  color: rgba(232, 236, 239, 0.78);
}

#home-btn:hover {
  border-color: rgba(0, 245, 212, 0.44);
  background: rgba(0, 245, 212, 0.075);
  color: #fff;
  box-shadow: 0 14px 38px rgba(0, 245, 212, 0.08);
}

#user-btn {
  padding: 0 16px;
  width: auto;
  min-width: 58px;
  height: 44px;
  border-radius: 24px;
  gap: 8px;
  font-size: 12px;
  letter-spacing: 0.4px;
}

.login-word {
  font-weight: 600;
}
</style>
