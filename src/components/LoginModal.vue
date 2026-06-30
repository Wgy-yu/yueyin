<script setup lang="ts">
import { ref, onUnmounted } from "vue";
import { useAccountStore } from "../stores/account";
import type { SourceType } from "../types/track";

const emit = defineEmits<{ (e: "close"): void }>();
const account = useAccountStore();
const tab = ref<SourceType>("netease");
const cookieInput = ref("");
const qrImg = ref("");
const qrKey = ref("");
const qrStatus = ref<"idle" | "loading" | "waiting" | "scanned" | "expired" | "success" | "error">("idle");
const qrStatusText = ref("");
const loginLoading = ref(false);
const loginError = ref("");
let pollTimer: ReturnType<typeof setInterval> | null = null;

async function startQr() {
  qrStatus.value = "loading";
  qrStatusText.value = "正在获取二维码...";
  try {
    const { key, img } = await account.startQrLogin();
    qrKey.value = key;
    qrImg.value = img;
    qrStatus.value = "waiting";
    qrStatusText.value = "请使用网易云音乐 APP 扫码";
    startPolling();
  } catch {
    qrStatus.value = "error";
    qrStatusText.value = "获取二维码失败";
  }
}

function startPolling() {
  stopPolling();
  pollTimer = setInterval(async () => {
    if (!qrKey.value) return;
    try {
      const resp = await account.pollQr(qrKey.value);
      if (resp.success) {
        qrStatus.value = "success";
        qrStatusText.value = resp.profile?.nickname ? `登录成功：${resp.profile.nickname}` : "登录成功！";
        stopPolling();
        setTimeout(() => emit("close"), 1000);
      } else if (resp.code === 800) {
        qrStatus.value = "expired";
        qrStatusText.value = "二维码已过期，点击刷新";
        stopPolling();
      } else if (resp.code === 802) {
        qrStatus.value = "scanned";
        qrStatusText.value = "已扫码，请在手机上确认";
      }
    } catch {
      // keep polling
    }
  }, 2000);
}

function stopPolling() {
  if (pollTimer) { clearInterval(pollTimer); pollTimer = null; }
}

async function submitCookie() {
  if (!cookieInput.value.trim()) return;
  loginLoading.value = true;
  loginError.value = "";
  try {
    const info = await account.loginCookie(cookieInput.value.trim(), tab.value);
    if (info.loggedIn) {
      emit("close");
    } else {
      loginError.value = "登录失败，请检查 cookie";
    }
  } catch (e) {
    loginError.value = String(e);
  } finally {
    loginLoading.value = false;
  }
}

onUnmounted(stopPolling);
</script>

<template>
  <div class="login-mask fixed inset-0 z-50 flex items-center justify-center" @click.self="emit('close')">
    <div class="login-modal w-[min(470px,92vw)] rounded-[18px] p-8 text-center font-sans">
      <!-- Header -->
      <div class="mb-4">
        <div class="text-[17px] font-bold text-yueyin-ink">登录账号</div>
      </div>

      <!-- Platform tabs -->
      <div class="mx-auto mb-4 flex gap-2 rounded-full border border-white/[0.075] bg-white/[0.035] p-1">
        <button
          class="h-8 flex-1 rounded-full text-[11.5px] font-bold tracking-wider transition-all"
          :class="tab === 'netease' ? 'bg-[rgba(217,91,103,.16)] text-[#ffd7dc]' : 'text-white/50 hover:text-white/80'"
          @click="tab = 'netease'; qrStatus = 'idle'; stopPolling()"
        >网易云</button>
        <button
          class="h-8 flex-1 rounded-full text-[11.5px] font-bold tracking-wider transition-all"
          :class="tab === 'qq' ? 'bg-[rgba(191,214,107,.16)] text-[#f3ffd1]' : 'text-white/50 hover:text-white/80'"
          @click="tab = 'qq'; qrStatus = 'idle'; stopPolling()"
        >QQ 音乐</button>
      </div>

      <!-- Intro card -->
      <div class="login-intro -mx-1.5 mb-4 rounded-[14px] border border-white/[0.075] px-[15px] py-[14px] text-left">
        <div class="mb-1.5 text-[10px] font-bold uppercase tracking-[0.18em] text-[rgba(244,210,138,.72)]">Melovibe</div>
        <div class="mb-1.5 text-[18px] font-bold leading-[1.18] text-[rgba(255,255,255,.95)]">音乐播放器，也是一座视觉舞台</div>
        <div class="text-[12px] leading-[1.58] text-[rgba(255,255,255,.56)]">搜索或导入一首歌即可播放；登录后会同步歌单、红心和播客，让封面、歌词和粒子跟着音乐动起来。</div>
      </div>

      <!-- Netease QR -->
      <div v-if="tab === 'netease'" class="mb-4">
        <div class="qr-area flex flex-col items-center gap-3 py-3">
          <!-- QR container: 200x200, rounded 16px -->
          <div class="qr-shell relative flex h-[200px] w-[200px] items-center justify-center rounded-2xl border border-[rgba(217,91,103,.28)] bg-[radial-gradient(circle_at_50%_42%,rgba(217,91,103,.22),transparent_46%),rgba(255,255,255,.035)]">
            <div v-if="qrStatus === 'idle'">
              <button class="rounded-xl border border-[rgba(217,91,103,.3)] bg-[rgba(217,91,103,.1)] px-5 py-2.5 text-[13px] font-semibold text-[#ffd7dc] transition-colors hover:bg-[rgba(217,91,103,.18)]" @click="startQr">生成二维码</button>
            </div>
            <div v-else-if="qrStatus === 'loading'" class="qr-spinner h-7 w-7 rounded-full border-2 border-white/10 border-t-[rgba(217,91,103,.7)]"></div>
            <template v-else>
              <img :src="qrImg" alt="登录二维码" class="h-full w-full rounded-xl bg-white p-2" />
              <div v-if="qrStatus === 'expired'" class="absolute inset-0 flex cursor-pointer items-center justify-center rounded-2xl bg-black/70 text-sm font-semibold text-white" @click="startQr">点击刷新</div>
            </template>
          </div>
          <!-- Status text: min-h 18px, mb 14px -->
          <div
            class="min-h-[18px] text-center text-[12px]"
            :class="{
              'text-white/55': qrStatus === 'idle' || qrStatus === 'loading' || qrStatus === 'waiting',
              'text-[rgba(0,245,212,.9)]': qrStatus === 'success' || qrStatus === 'scanned',
              'text-[rgba(255,100,120,.9)]': qrStatus === 'error',
              'text-[rgba(255,200,100,.8)]': qrStatus === 'expired',
            }"
          >{{ qrStatusText }}</div>
        </div>
      </div>

      <!-- QQ Cookie login -->
      <div v-else class="mb-4 text-left">
        <div class="mb-3 text-[12px] text-white/50">QQ 音乐暂使用 Cookie 登录，后续版本将支持网页扫码。</div>
        <textarea
          v-model="cookieInput"
          class="mb-2 w-full resize-y rounded-[10px] border border-white/10 bg-white/[0.04] p-3 font-mono text-[12px] text-white/90 outline-none transition-colors focus:border-[rgba(0,245,212,.4)]"
          placeholder="uin=xxx; qm_keyst=xxx"
          rows="3"
        ></textarea>
        <div v-if="loginError" class="mb-2 text-[12px] text-[rgba(255,100,120,.9)]">{{ loginError }}</div>
        <button
          class="h-9 w-full rounded-[10px] border border-[rgba(244,210,138,.25)] bg-[rgba(244,210,138,.08)] text-[12.5px] font-semibold text-[rgba(244,210,138,.9)] transition-colors hover:bg-[rgba(244,210,138,.14)] disabled:opacity-50"
          :disabled="loginLoading"
          @click="submitCookie"
        >{{ loginLoading ? "登录中..." : "保存 Cookie" }}</button>
      </div>

      <!-- Bottom actions -->
      <div class="flex flex-wrap justify-center gap-2.5">
        <button class="rounded-lg border border-white/10 bg-white/[0.04] px-5 py-2 text-[12.5px] text-white/60 transition-colors hover:bg-white/[0.08]" @click="emit('close')">取消</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.login-mask {
  background: rgba(0, 0, 0, 0.78);
  backdrop-filter: blur(12px);
}

.login-modal {
  background: linear-gradient(180deg, rgba(24, 23, 26, 0.96), rgba(12, 11, 12, 0.92));
  border: 1px solid rgba(244, 210, 138, 0.16);
  box-shadow: 0 26px 90px rgba(0, 0, 0, 0.56), inset 0 1px 0 rgba(255, 255, 255, 0.06);
}

.login-intro {
  background: linear-gradient(135deg, rgba(255, 83, 103, 0.09), rgba(244, 210, 138, 0.055), rgba(255, 255, 255, 0.025));
}

.qr-spinner {
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}
</style>
