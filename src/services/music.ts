import type { Track, SourceType } from "../types/track";

const API_BASE = "/api";

// ponytail: fetch wrapper, no axios dependency
async function api<T>(path: string, params?: Record<string, string>): Promise<T> {
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

export async function searchSongs(keyword: string, source: SourceType = "netease"): Promise<Track[]> {
  const endpoint = source === "qq" ? "/qq/search" : "/search";
  const data = await api<{ result?: { songs?: RawSong[] }; data?: { song?: { list?: RawSong[] } } }>(
    endpoint,
    { keywords: keyword, limit: "30" }
  );
  const rawSongs = data.result?.songs ?? data.data?.song?.list ?? [];
  return rawSongs.map((s) => normalizeSong(s, source));
}

export async function getSongUrl(id: string, source: SourceType = "netease"): Promise<string | null> {
  const endpoint = source === "qq" ? "/qq/song/url" : "/song/url";
  try {
    const data = await api<{ data?: { url?: string }[]; req_0?: { data?: { midurlinfo?: { purl?: string }[] } } }>(
      endpoint,
      { id, br: "320000" }
    );
    return data.data?.[0]?.url ?? data.req_0?.data?.midurlinfo?.[0]?.purl ?? null;
  } catch {
    return null;
  }
}

export function proxiedAudioUrl(url: string): string {
  if (!url) return "";
  return `${API_BASE}/audio?url=${encodeURIComponent(url)}`;
}
