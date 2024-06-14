/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./src/**/*.{js,jsx,ts,tsx}",
  ],
  theme: {
    extend: {
          gridTemplateColumns: {
            'custom-row-layout': '10% repeat(3, 1fr) 20%'
          }
        }
  },
  plugins: [],
}

