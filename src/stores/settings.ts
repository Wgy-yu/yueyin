import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke, isTauri } from "@tauri-apps/api/core";

export interface Settings {
  volume: string;
  quality: string;
  playMode: string;
  [key: string]: string; // ponytail: flexible KV for search history, lyrics cache, etc.
}

const DEFAULTS: Settings = {
  volume: "0.7",
  quality: "standard",
  playMode: "sequence",
};

export const useSettingsStore = defineStore("settings", () => {
  const settings = ref<Settings>({ ...DEFAULTS });
  const loaded = ref(false);

  async function load() {
    if (!isTauri()) {
      // ponytail: browser dev fallback, no backend available
      try {
        const raw = localStorage.getItem("yueyin-settings");
        if (raw) settings.value = { ...DEFAULTS, ...JSON.parse(raw) };
      } catch {}
      loaded.value = true;
      return;
    }
    const keys = Object.keys(DEFAULTS) as (keyof Settings)[];
    for (const key of keys) {
      try {
        const val = await invoke<string | null>("get_setting", { key });
        if (val !== null) settings.value[key] = val;
      } catch (e) {
        console.warn(`读取设置 ${key} 失败:`, e);
      }
    }
    loaded.value = true;
  }

  async function set<K extends keyof Settings>(key: K, value: Settings[K]) {
    settings.value[key] = value;
    if (!isTauri()) {
      localStorage.setItem("yueyin-settings", JSON.stringify(settings.value));
      return;
    }
    try {
      await invoke("set_setting", { key, value });
    } catch (e) {
      console.warn(`写入设置 ${key} 失败:`, e);
    }
  }

  function get<K extends keyof Settings>(key: K): Settings[K] {
    return settings.value[key];
  }

  return { settings, loaded, load, set, get };
});
