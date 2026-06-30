export interface LyricLine {
  t: number;      // start time in seconds
  duration: number;
  text: string;
}

// Standard LRC [mm:ss.xx]text
const LRC_RE = /\[(\d{1,2}):(\d{1,2})(?:\.(\d{1,3}))?\]/g;
const YRC_LINE_RE = /^\[(\d+),(\d+)\](.*)$/;
const YRC_WORD_RE = /\(\d+,\d+(?:,\d+)?\)/g;

export function parseLrc(text: string): LyricLine[] {
  const lines: LyricLine[] = [];
  for (const raw of text.split(/\r?\n/)) {
    if (!raw.trim()) continue;
    const yrc = raw.match(YRC_LINE_RE);
    if (yrc) {
      const content = yrc[3].replace(YRC_WORD_RE, "").trim();
      if (content) lines.push({
        t: Number(yrc[1]) / 1000,
        duration: Math.max(0.45, Number(yrc[2]) / 1000),
        text: content,
      });
      continue;
    }
    const tags: number[] = [];
    let match;
    LRC_RE.lastIndex = 0;
    while ((match = LRC_RE.exec(raw)) !== null) {
      const min = parseInt(match[1], 10);
      const sec = parseInt(match[2], 10);
      const frac = match[3] ?? "0";
      const t = min * 60 + sec + parseFloat(`0.${frac}`);
      tags.push(t);
    }
    const content = raw.replace(LRC_RE, "").trim();
    if (!content || !tags.length) continue;
    for (const t of tags) {
      lines.push({ t, duration: 0, text: content });
    }
  }
  lines.sort((a, b) => a.t - b.t);
  // Infer duration from next line
  for (let i = 0; i < lines.length; i++) {
    const next = lines[i + 1];
    const dur = next ? next.t - lines[i].t : 4.8;
    if (!lines[i].duration) lines[i].duration = Math.max(0.45, Math.min(12, dur));
  }
  return lines;
}

// ponytail: plain text fallback — evenly space lines across duration
export function parsePlainText(text: string, duration: number): LyricLine[] {
  const rows = text.split(/\r?\n/).filter((l) => l.trim());
  if (!rows.length) return [];
  const gap = Math.max(2.8, Math.min(7.2, duration / rows.length));
  return rows.map((text, i) => ({ t: i * gap, duration: gap, text: text.trim() }));
}
