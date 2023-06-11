
/** @type {import('tailwindcss').Config} */

module.exports = {
  content: {
    relative: true,
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      colors: {
        'royal-blue': {
          '50': '#f0f8fe',
          '100': '#dceffd',
          '200': '#c2e3fb',
          '300': '#97d3f9',
          '400': '#66baf4',
          '500': '#439cee',
          '600': '#3584e4',
          '700': '#256ad0',
          '800': '#2455a9',
          '900': '#224a86',
          '950': '#192e52',
        },
      }
    },
  },
  plugins: [],
}
