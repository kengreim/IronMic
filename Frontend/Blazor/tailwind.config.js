/** @type {import('tailwindcss').Config} */

const defaultTheme = require('tailwindcss/defaultTheme')

module.exports = {
    content: [
        "./**/*.{razor,css,scss,cs,js,html,cshtml}",
        "./node_modules/flowbite/**/*.js"
    ],
    theme: {
        extend: {
            fontFamily: {
                sans: ['"Public Sans"', ...defaultTheme.fontFamily.sans],
                mono: ["IBM Plex Mono", ...defaultTheme.fontFamily.mono],
                heading: ['"Barlow Condensed"', ...defaultTheme.fontFamily.sans]
            }
        },
    },
    plugins: [
        require('flowbite/plugin')
    ],
}