/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{vue,js,ts,jsx,tsx,svelte}", "./settings.html"],
  theme: {
    extend: {
      fontFamily: {
        inter: ["Inter", "sans-serif"],
        godomaum: ["godoMaum", "sans-serif"]
      }
    },
  },
  plugins: [],
}

