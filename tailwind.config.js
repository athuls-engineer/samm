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
          main: "#0f1115",
          subtle: "#16181d",
          card: "#1c1f26",
          hover: "#252932",
          active: "#2e333e",
        },
        border: {
          subtle: "#2a2e39",
          DEFAULT: "#383e4d",
          strong: "#4f576c",
        },
        accent: {
          DEFAULT: "#f59e0b",
          hover: "#d97706",
          muted: "rgba(245, 158, 11, 0.15)",
        },
      },
    },
  },
  plugins: [],
}
