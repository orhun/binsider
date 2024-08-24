import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";
import tailwind from "@astrojs/tailwind";

// https://astro.build/config
export default defineConfig({
  site: "https://binsider.dev",
  description: "Analyze ELF binaries like a boss.",
  integrations: [
    starlight({
      title: "Binsider",
      social: {
        github: "https://github.com/orhun/binsider",
        mastodon: "https://fosstodon.org/@orhun",
        "x.com": "https://twitter.com/orhundev",
        linkedin: "https://www.linkedin.com/in/orhunp",
      },
      logo: {
        dark: "./src/assets/binsider-logo-dark.png",
        light: "./src/assets/binsider-logo-light.png",
        replacesTitle: true,
      },
      components: {
        Head: "./src/components/Head.astro",
        Header: "./src/components/Header.astro",
        Header: "./src/components/Header.astro",
        Hero: "./src/components/Hero.astro",
        Footer: "./src/components/Footer.astro",
      },
      customCss: [
        "@fontsource/dejavu-sans/400.css",
        "./src/tailwind.css",
        "./src/styles/custom.css",
      ],
      sidebar: [
        {
          label: "Getting Started",
          autogenerate: {
            directory: "getting-started",
          },
        },
        {
          label: "Installation",
          autogenerate: {
            directory: "installation",
          },
        },
        {
          label: "Usage",
          autogenerate: {
            directory: "usage",
          },
        },
        "pricing",
      ],
      editLink: {
        baseUrl: "https://github.com/orhun/binsider/edit/main/website",
      },
    }),
    tailwind({ applyBaseStyles: false }),
  ],
});
