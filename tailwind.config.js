/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: 'jit',
  content: ["./templates/**/*.{html,js,rs}"],
  darkMode: 'media', // or 'class'
  theme: {
    extend: {
      colors: {
        primary: "#6C0345",
        secondary: "#DC6B19",
        tierciary: "#F7C566",
        fourthiary: "#FFF8DC"
      }

    },
  },
  plugins: [],
}
