/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./src/**/*.{js,jsx,ts,tsx}",
  ],
  theme: {
    extend: {
        backgroundColor: {
            'neutral-700-95': 'rgba(64, 64, 64, .95)'
        },
        gridTemplateColumns: {
          'custom-row-layout': '10% repeat(3, 1fr) 20%'
        }
    }
  },
  plugins: [],
}

