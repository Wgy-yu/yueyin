import type { Track, SourceType } from "../types/track";

// Detect Tauri environment
const isTauri = "__TAURI__" in window || "__TAURI_INTERNALS__" in window;

// Lazy Tauri invoke — only imported when needed
let tauriInvoke: ((cmd: string, args?: Record<string, unknown>) => Promise<unknown>) | null = null;

async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (!tauriInvoke) {
    const mod = await import("@tauri-apps/api/core");
    tauriInvoke = mod.invoke;
  }
  return tauriInvoke(cmd, args) as Promise<T>;
}

const API_BASE = "/api";

// HTTP fallback for non-Tauri (dev server / Mineradio proxy)
async function httpApi<T>(path: string, params?: Record<string, string>): Promise<T> {
  const url = new URL(API_BASE + path, window.location.origin);
  if (params) Object.entries(params).forEach(([k, v]) => url.searchParams.set(k, v));
  const res = await fetch(url.toString());
  if (!res.ok) throw new Error(`API ${path} failed: ${res.status}`);
  return res.json();
}

interface RawSong {
  id?: number | string;
  mid?: string;
  name?: string;
  song?: string;
  artists?: { name: string }[];
  singer?: { name: string }[];
  artist?: string;
  album?: { name?: string; picUrl?: string };
  albumname?: string;
  img?: string;
  duration?: number;
  dt?: number;
  interval?: number;
  source?: SourceType;
}

function normalizeSong(raw: RawSong, source: SourceType): Track {
  const id = String(raw.id ?? raw.mid ?? "");
  const name = raw.name ?? raw.song ?? "未知";
  let artist = "未知";
  if (raw.artists?.length) artist = raw.artists.map((a) => a.name).join("/");
  else if (raw.singer?.length) artist = raw.singer.map((s) => s.name).join("/");
  else if (raw.artist) artist = raw.artist;

  const album = raw.album?.name ?? raw.albumname;
  const coverUrl = raw.album?.picUrl ?? raw.img;
  const duration = raw.duration ?? raw.dt ?? raw.interval;

  return { id, name, artist, album, coverUrl, duration: duration ? Math.floor(duration / 1000) : undefined, source };
}

// ---------- Search ----------

export async function searchSongs(keyword: string, source: SourceType = "netease"): Promise<Track[]> {
  if (isTauri) {
    const data = await invoke<{ songs?: RawSong[]; provider?: string }>("music_search", {
      keywords: keyword,
      limit: 30,
      source,
    });
    const rawSongs = data.songs ?? [];
    return rawSongs.map((s) => normalizeSong(s, source));
  }

  const endpoint = source === "qq" ? "/qq/search" : "/search";
  const data = await httpApi<{ result?: { songs?: RawSong[] }; data?: { song?: { list?: RawSong[] } } }>(
    endpoint,
    { keywords: keyword, limit: "30" }
  );
  const rawSongs = data.result?.songs ?? data.data?.song?.list ?? [];
  return rawSongs.map((s) => normalizeSong(s, source));
}

// ---------- Song URL ----------

export async function getSongUrl(id: string, source: SourceType = "netease"): Promise<string | null> {
  if (isTauri) {
    try {
      const data = await invoke<{ url?: string; playable?: boolean }>("music_song_url", {
        id,
        source,
        quality: "hires",
      });
      return data.url ?? null;
    } catch {
      return null;
    }
  }

  const endpoint = source === "qq" ? "/qq/song/url" : "/song/url";
  try {
    const data = await httpApi<{ data?: { url?: string }[]; req_0?: { data?: { midurlinfo?: { purl?: string }[] } } }>(
      endpoint,
      { id, br: "320000" }
    );
    return data.data?.[0]?.url ?? data.req_0?.data?.midurlinfo?.[0]?.purl ?? null;
  } catch {
    return null;
  }
}

// ---------- Audio proxy ----------

export function proxiedAudioUrl(url: string): string {
  if (!url) return "";
  if (isTauri) {
    // In Tauri mode, return the direct URL — the audio element can fetch it directly.
    // The audio_proxy command is available for CORS-restricted scenarios via invoke.
    return url;
  }
  return `${API_BASE}/audio?url=${encodeURIComponent(url)}`;
}

/** Fetch audio bytes via Tauri command (for CORS-restricted URLs). Returns a Blob URL. */
export async function fetchAudioBlobUrl(url: string): Promise<string | null> {
  if (!isTauri) return null;
  try {
    const bytes = await invoke<number[]>("music_audio_proxy", { url });
    const blob = new Blob([new Uint8Array(bytes)]);
    return URL.createObjectURL(blob);
  } catch {
    return null;
  }
}

// ---------- Lyrics ----------

export async function fetchLyrics(id: string, source: SourceType = "netease"): Promise<string | null> {
  if (isTauri) {
    try {
      const data = await invoke<{ lyric?: string; tlyric?: string; yrc?: string; qrc?: string }>("music_lyric", {
        id,
        source,
      });
      return data.yrc ?? data.lyric ?? null;
    } catch {
      return null;
    }
  }

  const endpoint = source === "qq" ? "/qq/lyric" : "/lyric";
  const param = source === "qq" ? "mid" : "id";
  try {
    const data = await httpApi<{ lrc?: { lyric?: string }; yrc?: { lyric?: string }; lyric?: string }>(
      endpoint,
      { [param]: id }
    );
    return data.yrc?.lyric ?? data.lrc?.lyric ?? data.lyric ?? null;
  } catch {
    return null;
  }
}

// ---------- Login ----------

export interface LoginInfo {
  loggedIn: boolean;
  userId?: string;
  nickname: string;
  avatar: string;
  vipType?: number;
  vipLevel?: string;
  isVip?: boolean;
  isSvip?: boolean;
  vipLabel?: string;
}

export async function getLoginStatus(source: SourceType = "netease"): Promise<LoginInfo> {
  const cmd = source === "qq" ? "music_qq_login_status" : "music_login_status";
  return invoke<LoginInfo>(cmd);
}

export async function loginWithCookie(cookie: string, source: SourceType = "netease"): Promise<LoginInfo> {
  const cmd = source === "qq" ? "music_qq_login_cookie" : "music_login_cookie";
  return invoke<LoginInfo>(cmd, { cookie });
}

export async function logout(source: SourceType = "netease"): Promise<void> {
  const cmd = source === "qq" ? "music_qq_logout" : "music_logout";
  await invoke(cmd);
}

export async function getQrKey(): Promise<string> {
  const data = await invoke<{ key: string }>("music_qr_key");
  return data.key;
}

export async function createQrCode(key: string): Promise<{ img: string; url: string }> {
  return invoke<{ img: string; url: string }>("music_qr_create", { key });
}

export async function checkQrStatus(key: string): Promise<{ code: number; success: boolean; loggedIn?: boolean; profile?: { nickname: string; avatar: string } }> {
  return invoke("music_qr_check", { key });
}

// ---------- Playlists ----------

export interface PlaylistInfo {
  id: string;
  name: string;
  cover: string;
  trackCount: number;
  playCount: number;
  creator: string;
  subscribed: boolean;
  specialType: number;
  provider: string;
}

export async function getUserPlaylists(uid?: string, source: SourceType = "netease"): Promise<PlaylistInfo[]> {
  const data = await invoke<{ playlists: PlaylistInfo[] }>("music_user_playlists", {
    uid: uid ?? "",
    source,
  });
  return data.playlists ?? [];
}

export async function getPlaylistTracks(id: string, source: SourceType = "netease"): Promise<{ playlist: PlaylistInfo; tracks: Track[] }> {
  const data = await invoke<{ playlist: PlaylistInfo; tracks: RawSong[] }>("music_playlist_tracks", {
    id,
    source,
  });
  return {
    playlist: data.playlist,
    tracks: (data.tracks ?? []).map((s) => normalizeSong(s, source)),
  };
}

// ---------- Favorites ----------

export async function checkLiked(ids: string[]): Promise<Record<string, boolean>> {
  const data = await invoke<{ liked: Record<string, boolean> }>("music_like_check", { ids });
  return data.liked ?? {};
}

export async function toggleLike(id: string, like: boolean): Promise<boolean> {
  const data = await invoke<{ ok: boolean }>("music_like_toggle", { id, like });
  return data.ok;
}
