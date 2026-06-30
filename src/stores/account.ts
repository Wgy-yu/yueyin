import { defineStore } from "pinia";
import { ref } from "vue";
import type { SourceType } from "../types/track";
import type { LoginInfo, PlaylistInfo } from "../services/music";
import {
  getLoginStatus, loginWithCookie, logout as apiLogout,
  openWebLogin as apiOpenWebLogin,
  getQrKey, createQrCode, checkQrStatus,
  getUserPlaylists, getPlaylistTracks,
} from "../services/music";
import type { Track } from "../types/track";

export const useAccountStore = defineStore("account", () => {
  const netease = ref<LoginInfo>({ loggedIn: false, nickname: "", avatar: "" });
  const qq = ref<LoginInfo>({ loggedIn: false, nickname: "", avatar: "" });
  const playlists = ref<PlaylistInfo[]>([]);
  const playlistTracks = ref<Track[]>([]);
  const playlistLoading = ref(false);
  const activeSource = ref<SourceType>("netease");

  const current = () => activeSource.value === "qq" ? qq.value : netease.value;

  async function refreshStatus() {
    try {
      netease.value = await getLoginStatus("netease");
    } catch { netease.value = { loggedIn: false, nickname: "", avatar: "" }; }
    try {
      qq.value = await getLoginStatus("qq");
    } catch { qq.value = { loggedIn: false, nickname: "", avatar: "" }; }
  }

  async function loginCookie(cookie: string, source: SourceType = "netease") {
    const info = await loginWithCookie(cookie, source);
    if (source === "qq") qq.value = info;
    else netease.value = info;
    return info;
  }

  async function doLogout(source: SourceType = "netease") {
    await apiLogout(source);
    if (source === "qq") qq.value = { loggedIn: false, nickname: "", avatar: "" };
    else netease.value = { loggedIn: false, nickname: "", avatar: "" };
    playlists.value = [];
  }

  async function openWebLogin(source: SourceType = "netease") {
    const info = await apiOpenWebLogin(source);
    if (source === "qq") qq.value = info;
    else netease.value = info;
    return info;
  }

  // QR login: get key + QR image
  async function startQrLogin(): Promise<{ key: string; img: string }> {
    const key = await getQrKey();
    const { img } = await createQrCode(key);
    return { key, img };
  }

  // QR login: poll status. Returns code (800/801/802/803) and profile on success.
  async function pollQr(key: string): Promise<{ code: number; success: boolean; profile?: { nickname: string; avatar: string } }> {
    const resp = await checkQrStatus(key);
    if (resp.success && resp.loggedIn) {
      await refreshStatus();
      return { code: resp.code, success: true, profile: resp.profile };
    }
    return { code: resp.code, success: false };
  }

  async function fetchPlaylists(source?: SourceType) {
    const src = source ?? activeSource.value;
    const info = src === "qq" ? qq.value : netease.value;
    playlists.value = await getUserPlaylists(info.userId ?? "", src);
  }

  async function fetchPlaylistTracks(id: string, source?: SourceType) {
    const src = source ?? activeSource.value;
    playlistLoading.value = true;
    try {
      const result = await getPlaylistTracks(id, src);
      playlistTracks.value = result.tracks;
    } finally {
      playlistLoading.value = false;
    }
  }

  return {
    netease, qq, playlists, playlistTracks, playlistLoading, activeSource,
    current, refreshStatus, loginCookie, doLogout, openWebLogin,
    startQrLogin, pollQr, fetchPlaylists, fetchPlaylistTracks,
  };
});
