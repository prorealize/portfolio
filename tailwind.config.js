const defaultTheme = require("tailwindcss/defaultTheme");

module.exports = {
    mode: "jit",
    content: {
      files: ["src/**/*.rs", "**/*.html"],
    },
    safelist: ["active"],
    theme: {
    fontFamily: {
      header: ["Raleway", "sans-serif"],
      body: ["Open Sans", "sans-serif"],
    },

    screens: {
      xs: "375px",
      ...defaultTheme.screens,
    },

    colors: {
      transparent: "transparent",
      primary: "#0F52BA",
      white: "#ffffff",
      black: "#000000",
      yellow: "#f9e71c",
      "grey-20": "#7c7c7c",
      "grey-50": "#f4f3f8",
      "hero-gradient-from": "rgba(30, 64, 174, 0.95)",
      "hero-gradient-to": "rgba(15, 82, 186, 0.93)",
    },

    container: {
      center: true,
      padding: "1rem",
    },
    
  },
    variants: {
      extend: {},
    },
    plugins: [
        require("@tailwindcss/typography"),
        require("@tailwindcss/forms"),
        require("@tailwindcss/aspect-ratio"),
    ],
  };