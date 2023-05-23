module.exports = {
  content: [ "./src/**/*.rs", "./index.html" ],
  theme: {
    fontFamily: {
      sans: ["Work Sans", "sans-serif"],
    }
  },
  plugins: [require('@tailwindcss/forms'), require("@tailwindcss/typography")]
};
