/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{vue,js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        yueyin: {
          bg: "#08090B",
          paper: "#0E1014",
          ink: "#E8ECEF",
          "ink-2": "#D2D7DC",
          muted: "#8A9099",
          hair: "#1A1D22",
          "hair-2": "#262A31",
          accent: "#00F5D4",
          "accent-hover": "#00E0BE",
          blue: "#2442FF",
          warm: "#F8F4EE",
        },
      },
      fontFamily: {
        sans: [
          "Noto Sans SC",
          "PingFang SC",
          "HarmonyOS Sans SC",
          "Inter",
          "system-ui",
          "sans-serif",
        ],
        mono: ["JetBrains Mono", "Geist Mono", "SF Mono", "ui-monospace", "monospace"],
      },
      borderRadius: {
        window: "34px",
      },
      boxShadow: {
        glass: "0 22px 64px rgba(0,0,0,.30), 0 0 34px rgba(0,245,212,.052), inset 0 1px 0 rgba(255,255,255,.16), inset 0 -24px 58px rgba(0,0,0,.16)",
        "glass-focus": "0 24px 72px rgba(0,0,0,.34), 0 0 0 1px rgba(0,245,212,.13), 0 0 42px rgba(0,245,212,.075), inset 0 1px 0 rgba(255,255,255,.20)",
      },
    },
  },
  plugins: [],
};
