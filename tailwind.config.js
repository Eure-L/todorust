/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: 'jit',
  content: ["./templates/**/*.{html,js,rs}"],
  darkMode: 'media', // or 'class'
  theme: {
    extend: {
      spacing: {
        '5px': '5px',
        '100px': '100px',
      },
      colors: {
        primary: "#153448",
        secondary: "#3C5B6F",
        tierciary: "#948979",
        fourthiary: "#DFD0B8"
      }

    },
  },
  plugins: [],
}
