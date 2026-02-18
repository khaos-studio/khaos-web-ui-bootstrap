export default defineNuxtConfig({
  modules: ["@nuxt/ui", "@pinia/nuxt"],
  ssr: false,
  css: ["@khaos/shared/styles"],
  devServer: {
    port: process.env.DEV_PORT_PROJECTS
      ? parseInt(process.env.DEV_PORT_PROJECTS)
      : 5173,
  },
});
