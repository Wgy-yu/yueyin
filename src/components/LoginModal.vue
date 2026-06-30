<script setup lang="ts">
import { onUnmounted, ref } from "vue";
import { useAccountStore } from "../stores/account";
import { cancelWebLogin } from "../services/music";
import type { SourceType } from "../types/track";

const emit = defineEmits<{ (e: "close"): void }>();
const account = useAccountStore();
const tab = ref<SourceType>("netease");
const status = ref<"idle" | "opening" | "waiting" | "success" | "error">("idle");
const statusText = ref("点击“网页登录”打开网易云官方窗口");
const cookieInput = ref("");
const showCookie = ref(false);
const loginError = ref("");

function selectProvider(provider: SourceType) {
  if (status.value === "waiting") return;
  tab.value = provider;
  status.value = "idle";
  statusText.value = provider === "qq"
    ? "点击“扫码登录”打开 QQ 音乐官方窗口"
    : "点击“网页登录”打开网易云官方窗口";
  showCookie.value = false;
  loginError.value = "";
}

async function openOfficialLogin() {
  if (status.value === "opening" || status.value === "waiting") return;
  status.value = "opening";
  statusText.value = "正在打开官方登录窗口…";
  try {
    status.value = "waiting";
    statusText.value = `已打开${tab.value === "qq" ? "QQ 音乐" : "网易云"}窗口，请在官方页面扫码登录…`;
    const info = await account.openWebLogin(tab.value);
    if (!info.loggedIn) throw new Error("官方会话验证失败");
    status.value = "success";
    statusText.value = `登录成功：${info.nickname || info.userId || "已同步账号"}`;
    setTimeout(() => emit("close"), 600);
  } catch (error) {
    status.value = "error";
    const message = String(error);
    statusText.value = message.includes("LOGIN_CANCELLED") ? "已取消登录" : message.replace(/^.*?: /, "");
  }
}

async function submitCookie() {
  if (!cookieInput.value.trim()) return;
  loginError.value = "";
  try {
    const info = await account.loginCookie(cookieInput.value.trim(), tab.value);
    if (!info.loggedIn) throw new Error("Cookie 无效或账号信息验证失败");
    emit("close");
  } catch (error) {
    loginError.value = String(error).replace(/^.*?: /, "");
  }
}

onUnmounted(() => { void cancelWebLogin(); });
</script>

<template>
  <div class="modal-mask" @click.self="emit('close')">
    <div class="modal dual-login-modal">
      <div class="login-platform-tabs">
        <button class="netease" :class="{ active: tab === 'netease' }" @click="selectProvider('netease')">网易云</button>
        <button class="qq" :class="{ active: tab === 'qq' }" @click="selectProvider('qq')">QQ 音乐</button>
      </div>

      <div class="login-intro">
        <div class="login-intro-kicker">Melovibe</div>
        <div class="login-intro-title">音乐播放器，也是一座视觉舞台</div>
        <div class="login-intro-body">搜索或导入一首歌即可播放；登录后会同步歌单、红心和播客，让封面、歌词和粒子跟着音乐动起来。</div>
      </div>

      <h2>扫码登录{{ tab === "qq" ? "QQ 音乐" : "网易云音乐" }}</h2>
      <div class="desc">打开 <b>{{ tab === "qq" ? "QQ 音乐" : "网易云音乐" }}官方网页登录窗口</b> 扫码，成功后会自动同步账号会话。</div>

      <div class="qr-shell web-login-preview" :class="tab === 'qq' ? 'qq-preview' : 'netease-preview'">
        <button class="web-login-mark" :disabled="status === 'opening' || status === 'waiting'" @click="openOfficialLogin">
          <b>{{ tab === "qq" ? "QQ" : "NE" }}</b>
          <span>{{ status === "waiting" ? "等待扫码确认" : "打开官方登录窗口" }}</span>
        </button>
      </div>

      <div class="login-status" :class="status">{{ statusText }}</div>

      <div v-if="showCookie" class="cookie-panel">
        <textarea v-model="cookieInput" spellcheck="false" autocomplete="off" :placeholder="tab === 'qq' ? 'uin=...; qqmusic_key=...; qm_keyst=...' : 'MUSIC_U=...; __csrf=...'" />
        <div v-if="loginError" class="cookie-error">{{ loginError }}</div>
        <button class="modal-btn primary" @click="submitCookie">保存</button>
      </div>

      <div class="btn-row">
        <button class="modal-btn" @click="emit('close')">取消</button>
        <button class="modal-btn" @click="emit('close')">先搜索一首歌</button>
        <button class="modal-btn" @click="showCookie = !showCookie">{{ showCookie ? "收起导入" : "手动导入" }}</button>
        <button class="modal-btn primary" :disabled="status === 'opening' || status === 'waiting'" @click="openOfficialLogin">
          {{ tab === "qq" ? "扫码登录" : "网页登录" }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.modal-mask{position:fixed;inset:0;z-index:50;background:radial-gradient(circle at 50% 48%,rgba(244,210,138,.07),transparent 34%),rgba(0,0,0,.78);display:flex;align-items:center;justify-content:center}
.modal{position:relative;background:linear-gradient(180deg,rgba(24,23,26,.96),rgba(12,11,12,.92));border:1px solid rgba(244,210,138,.16);border-radius:18px;padding:32px;text-align:center;box-shadow:0 26px 90px rgba(0,0,0,.56),0 0 0 1px rgba(255,255,255,.035),inset 0 1px 0 rgba(255,255,255,.07)}
.dual-login-modal{width:min(470px,92vw);max-width:min(470px,92vw)}
.modal h2{font-size:18px;font-weight:500;letter-spacing:.5px;margin-bottom:18px;color:rgba(255,255,255,.95)}
.modal .desc{font-size:12.5px;color:rgba(255,255,255,.5);margin-bottom:20px;line-height:1.6}
.login-platform-tabs{display:flex;gap:8px;margin:0 auto 18px;padding:4px;border-radius:999px;border:1px solid rgba(255,255,255,.075);background:rgba(255,255,255,.035)}
.login-platform-tabs button{flex:1;height:32px;border:0;border-radius:999px;background:transparent;color:rgba(255,255,255,.48);font:700 11.5px inherit;letter-spacing:.45px;cursor:pointer;transition:.22s}
.login-platform-tabs button.netease.active{background:rgba(217,91,103,.16);color:#ffd7dc}.login-platform-tabs button.qq.active{background:rgba(191,214,107,.16);color:#f3ffd1}
.login-intro{margin:-6px 0 16px;padding:14px 15px;border-radius:14px;border:1px solid rgba(255,255,255,.075);background:linear-gradient(135deg,rgba(255,83,103,.09),rgba(244,210,138,.055),rgba(255,255,255,.025));text-align:left}
.login-intro-kicker{font-size:10px;font-weight:780;letter-spacing:.18em;color:rgba(244,210,138,.72);text-transform:uppercase;margin-bottom:6px}.login-intro-title{font-size:18px;font-weight:760;line-height:1.18;color:rgba(255,255,255,.95);margin-bottom:6px}.login-intro-body{font-size:12px;line-height:1.58;color:rgba(255,255,255,.56)}
.qr-shell{position:relative;width:200px;height:200px;margin:0 auto 16px;border-radius:16px;display:flex;align-items:center;justify-content:center;padding:14px;background:rgba(255,255,255,.035);box-shadow:0 16px 42px rgba(0,0,0,.32),inset 0 1px 0 rgba(255,255,255,.08)}
.qr-shell.netease-preview{background:radial-gradient(circle at 50% 42%,rgba(217,91,103,.22),transparent 46%),rgba(255,255,255,.035);border:1px solid rgba(217,91,103,.28)}.qr-shell.qq-preview{background:radial-gradient(circle at 50% 42%,rgba(191,214,107,.22),transparent 46%),rgba(255,255,255,.035);border:1px solid rgba(191,214,107,.26)}
.web-login-mark{display:flex;width:100%;height:100%;border-radius:12px;align-items:center;justify-content:center;flex-direction:column;gap:8px;cursor:pointer;font-family:inherit;outline:none;transition:.22s}.netease-preview .web-login-mark{border:1px solid rgba(217,91,103,.34);color:#ffd7dc;background:linear-gradient(135deg,rgba(217,91,103,.10),rgba(244,210,138,.05))}.qq-preview .web-login-mark{border:1px solid rgba(191,214,107,.30);color:rgba(243,255,209,.88);background:linear-gradient(135deg,rgba(191,214,107,.10),rgba(157,184,207,.05))}.web-login-mark:disabled{opacity:.66;cursor:wait}.web-login-mark b{font-size:22px;letter-spacing:.08em}.web-login-mark span{font-size:10.5px;color:rgba(255,255,255,.46);letter-spacing:.5px}
.login-status{font-size:12px;color:rgba(255,255,255,.7);min-height:18px;margin-bottom:14px;letter-spacing:.3px}.login-status.success{color:#7ee2a8}.login-status.error{color:#f08080}.login-status.waiting,.login-status.opening{color:rgba(191,214,107,.86)}
.cookie-panel{margin-top:12px;padding:12px;border-radius:14px;border:1px solid rgba(255,255,255,.12);background:rgba(255,255,255,.035)}.cookie-panel textarea{width:100%;height:74px;resize:none;border:1px solid rgba(255,255,255,.08);border-radius:10px;background:rgba(0,0,0,.22);color:rgba(255,255,255,.82);padding:10px 11px;font:11px/1.45 inherit;outline:none}.cookie-error{font-size:11px;color:#f08080;margin:8px}
.btn-row{display:flex;flex-wrap:wrap;gap:10px;justify-content:center;margin-top:16px}.modal-btn{padding:8px 20px;font-size:12.5px;border-radius:8px;border:1px solid rgba(255,255,255,.15);background:rgba(255,255,255,.05);color:#fff;cursor:pointer;letter-spacing:.5px;transition:.2s;font-family:inherit}.modal-btn:hover{background:rgba(255,255,255,.12)}.modal-btn.primary{background:rgba(244,210,138,.14);border-color:rgba(244,210,138,.5);color:#f4d28a}.modal-btn:disabled{opacity:.55;cursor:wait}
</style>
