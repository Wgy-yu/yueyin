<script setup lang="ts">
import { ref, onUnmounted } from "vue";
import { useAccountStore } from "../stores/account";
import type { SourceType } from "../types/track";

const emit = defineEmits<{ (e: "close"): void }>();
const account = useAccountStore();
const tab = ref<SourceType>("netease");
const mode = ref<"qr" | "cookie">("qr");
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
  } catch (e) {
    qrStatus.value = "error";
    qrStatusText.value = "获取二维码失败";
  }
}

function startPolling() {
  stopPolling();
  pollTimer = setInterval(async () => {
    if (!qrKey.value) return;
    try {
      const { code, success } = await account.pollQr(qrKey.value);
      if (success) {
        qrStatus.value = "success";
        qrStatusText.value = "登录成功！";
        stopPolling();
        setTimeout(() => emit("close"), 800);
      } else if (code === 800) {
        qrStatus.value = "expired";
        qrStatusText.value = "二维码已过期，点击刷新";
        stopPolling();
      } else if (code === 802) {
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
  <div class="login-overlay" @click.self="emit('close')">
    <div class="login-modal">
      <div class="login-header">
        <div class="login-title">登录账号</div>
        <button class="login-close" @click="emit('close')" aria-label="关闭">×</button>
      </div>

      <div class="login-tabs">
        <button :class="{ active: tab === 'netease' }" @click="tab = 'netease'">网易云</button>
        <button :class="{ active: tab === 'qq' }" @click="tab = 'qq'">QQ 音乐</button>
      </div>

      <div class="login-modes">
        <button :class="{ active: mode === 'qr' }" @click="mode = 'qr'">扫码登录</button>
        <button :class="{ active: mode === 'cookie' }" @click="mode = 'cookie'">Cookie 登录</button>
      </div>

      <div v-if="mode === 'qr'" class="login-qr">
        <div v-if="tab === 'netease'" class="qr-area">
          <div v-if="qrStatus === 'idle'" class="qr-placeholder">
            <button class="qr-start-btn" @click="startQr">生成二维码</button>
          </div>
          <div v-else-if="qrStatus === 'loading'" class="qr-placeholder">
            <div class="qr-spinner"></div>
          </div>
          <div v-else class="qr-image-wrap">
            <img :src="qrImg" alt="登录二维码" class="qr-image" />
            <div v-if="qrStatus === 'expired'" class="qr-overlay" @click="startQr">
              <span>点击刷新</span>
            </div>
          </div>
          <div class="qr-status" :class="qrStatus">{{ qrStatusText }}</div>
        </div>
        <div v-else class="qr-unsupported">
          QQ 音乐暂不支持扫码登录，请使用 Cookie 方式
        </div>
      </div>

      <div v-else class="login-cookie">
        <div class="cookie-hint">
          {{ tab === 'netease' ? '粘贴网易云 Cookie（需包含 MUSIC_U）' : '粘贴 QQ 音乐 Cookie（需包含 uin）' }}
        </div>
        <textarea
          v-model="cookieInput"
          class="cookie-input"
          :placeholder="tab === 'netease' ? 'MUSIC_U=xxx; __csrf=xxx' : 'uin=xxx; qm_keyst=xxx'"
          rows="3"
        ></textarea>
        <div v-if="loginError" class="login-error">{{ loginError }}</div>
        <button class="cookie-submit" :disabled="loginLoading" @click="submitCookie">
          {{ loginLoading ? "登录中..." : "登录" }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.login-overlay {
  position: fixed;
  inset: 0;
  z-index: 100;
  background: rgba(0, 0, 0, 0.6);
  backdrop-filter: blur(8px);
  display: flex;
  align-items: center;
  justify-content: center;
}

.login-modal {
  width: min(420px, 90vw);
  background: linear-gradient(160deg, rgba(28, 32, 40, 0.96), rgba(12, 14, 18, 0.98));
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 20px;
  padding: 24px;
  box-shadow: 0 30px 80px rgba(0, 0, 0, 0.5);
  color: rgba(255, 255, 255, 0.88);
  font-family: inherit;
}

.login-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 18px;
}

.login-title {
  font-size: 17px;
  font-weight: 720;
}

.login-close {
  background: none;
  border: none;
  color: rgba(255, 255, 255, 0.5);
  font-size: 22px;
  cursor: pointer;
  padding: 4px 8px;
  border-radius: 8px;
  transition: background 0.15s;
}

.login-close:hover {
  background: rgba(255, 255, 255, 0.08);
  color: #fff;
}

.login-tabs, .login-modes {
  display: flex;
  gap: 4px;
  margin-bottom: 14px;
}

.login-tabs button, .login-modes button {
  flex: 1;
  height: 34px;
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 10px;
  background: rgba(255, 255, 255, 0.04);
  color: rgba(255, 255, 255, 0.6);
  font-family: inherit;
  font-size: 12px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.15s;
}

.login-tabs button.active, .login-modes button.active {
  background: rgba(0, 245, 212, 0.12);
  border-color: rgba(0, 245, 212, 0.35);
  color: rgba(0, 245, 212, 0.95);
}

.login-tabs button:hover, .login-modes button:hover {
  background: rgba(255, 255, 255, 0.08);
}

.qr-area {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  padding: 12px 0;
}

.qr-placeholder {
  width: 180px;
  height: 180px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: 1px dashed rgba(255, 255, 255, 0.12);
  border-radius: 14px;
}

.qr-start-btn {
  padding: 10px 20px;
  border-radius: 10px;
  border: 1px solid rgba(0, 245, 212, 0.3);
  background: rgba(0, 245, 212, 0.1);
  color: rgba(0, 245, 212, 0.9);
  font-family: inherit;
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.15s;
}

.qr-start-btn:hover {
  background: rgba(0, 245, 212, 0.18);
}

.qr-spinner {
  width: 28px;
  height: 28px;
  border: 2px solid rgba(255, 255, 255, 0.1);
  border-top-color: rgba(0, 245, 212, 0.7);
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin { to { transform: rotate(360deg); } }

.qr-image-wrap {
  position: relative;
  width: 180px;
  height: 180px;
}

.qr-image {
  width: 100%;
  height: 100%;
  border-radius: 10px;
  background: #fff;
  padding: 8px;
}

.qr-overlay {
  position: absolute;
  inset: 0;
  background: rgba(0, 0, 0, 0.7);
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  color: #fff;
  font-size: 14px;
  font-weight: 600;
}

.qr-status {
  font-size: 12px;
  color: rgba(255, 255, 255, 0.55);
  text-align: center;
}

.qr-status.success { color: rgba(0, 245, 212, 0.9); }
.qr-status.error { color: rgba(255, 100, 120, 0.9); }
.qr-status.expired { color: rgba(255, 200, 100, 0.8); }

.qr-unsupported {
  padding: 32px 0;
  text-align: center;
  font-size: 13px;
  color: rgba(255, 255, 255, 0.5);
}

.cookie-hint {
  font-size: 12px;
  color: rgba(255, 255, 255, 0.5);
  margin-bottom: 10px;
}

.cookie-input {
  width: 100%;
  background: rgba(255, 255, 255, 0.04);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 10px;
  padding: 10px 12px;
  color: rgba(255, 255, 255, 0.88);
  font-family: monospace;
  font-size: 12px;
  resize: vertical;
  outline: none;
  transition: border-color 0.15s;
}

.cookie-input:focus {
  border-color: rgba(0, 245, 212, 0.4);
}

.login-error {
  margin-top: 8px;
  font-size: 12px;
  color: rgba(255, 100, 120, 0.9);
}

.cookie-submit {
  margin-top: 12px;
  width: 100%;
  height: 38px;
  border: 1px solid rgba(0, 245, 212, 0.3);
  border-radius: 10px;
  background: rgba(0, 245, 212, 0.12);
  color: rgba(0, 245, 212, 0.95);
  font-family: inherit;
  font-size: 13px;
  font-weight: 680;
  cursor: pointer;
  transition: all 0.15s;
}

.cookie-submit:hover:not(:disabled) {
  background: rgba(0, 245, 212, 0.2);
}

.cookie-submit:disabled {
  opacity: 0.5;
  cursor: default;
}
</style>
