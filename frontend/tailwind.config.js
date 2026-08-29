/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        darkbase: '#06080e',
        cardbase: '#101726',
        accentblue: '#06b6d4',
        accentcyan: '#22d3ee',
        accentpurple: '#7c3aed',
        caboclogold: '#f59e0b',
        cabocloamber: '#d97706',
        cabocloemerald: '#059669',
      },
    },
  },
  plugins: [],
}
