const autoprefixer = require("autoprefixer");
const purgecss = require("@fullhuman/postcss-purgecss");
const tailwindcss = require("tailwindcss")("tailwind.config.js");

class TailwindExtractor {
    static extract(content) {
        return content.match(/[A-Za-z0-9-_:\/]+/g) || [];
    }
}

const development = {
    plugins: [tailwindcss, autoprefixer]
};

const production = {
    plugins: [
        tailwindcss,
        purgecss({
            extractors: [
                {
                    extensions: ["elm", "html", "js"],
                    extractor: TailwindExtractor
                }
            ],
            content: ["./src/**/*.elm", "./src/index.js", "./src/index.html"],
        }),
        autoprefixer
    ]
};

if (process.env.NODE_ENV === "production") {
    module.exports = production;
} else {
    module.exports = development;
}