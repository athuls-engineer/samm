/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        bg: {
          main: "var(--bg-main, #0f1115)",
          subtle: "var(--bg-subtle, #16181d)",
          card: "var(--bg-card, #1c1f26)",
          hover: "var(--bg-hover, #252932)",
          active: "var(--bg-active, #2e333e)",
        },
        border: {
          subtle: "var(--border-subtle, #2a2e39)",
          DEFAULT: "var(--border-default, #383e4d)",
          strong: "var(--border-strong, #4f576c)",
        },
        accent: {
          DEFAULT: "var(--accent-default, #f59e0b)",
          hover: "var(--accent-hover, #d97706)",
          muted: "var(--accent-muted, rgba(245, 158, 11, 0.15))",
        },
      },
    },
  },
  plugins: [],
}
