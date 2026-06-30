import { defineStore } from "pinia";
import { ref } from "vue";
import type { Track, SourceType } from "../types/track";
import { searchSongs } from "../services/music";
import { useSettingsStore } from "./settings";

export const useSearchStore = defineStore("search", () => {
  const query = ref("");
  const results = ref<Track[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const source = ref<SourceType>("netease");
  const history = ref<string[]>([]);
  let searchTimer: ReturnType<typeof setTimeout> | null = null;

  async function search(q: string) {
    const trimmed = q.trim();
    if (!trimmed) { results.value = []; return; }
    query.value = trimmed;
    loading.value = true;
    error.value = null;
    try {
      results.value = await searchSongs(trimmed, source.value);
      addToHistory(trimmed);
    } catch (e) {
      error.value = e instanceof Error ? e.message : "搜索失败";
      results.value = [];
    } finally {
      loading.value = false;
    }
  }

  function debouncedSearch(q: string) {
    if (searchTimer) clearTimeout(searchTimer);
    searchTimer = setTimeout(() => search(q), 350);
  }

  function addToHistory(q: string) {
    history.value = [q, ...history.value.filter((h) => h !== q)].slice(0, 10);
    // ponytail: persist via existing settings KV store, no new table
    try {
      useSettingsStore().set("searchHistory", JSON.stringify(history.value));
    } catch {}
  }

  async function loadHistory() {
    try {
      const settings = useSettingsStore();
      if (!settings.loaded) await settings.load();
      const raw = settings.get("searchHistory");
      if (raw) history.value = JSON.parse(raw);
    } catch {}
  }

  function clearResults() {
    results.value = [];
    query.value = "";
    error.value = null;
  }

  return {
    query, results, loading, error, source, history,
    search, debouncedSearch, clearResults, loadHistory,
  };
});
