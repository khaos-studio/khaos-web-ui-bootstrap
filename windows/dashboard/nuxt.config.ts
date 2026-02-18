export default defineNuxtConfig({
  modules: ["@nuxt/ui", "@pinia/nuxt"],
  ssr: false,
  css: ["@khaos/shared/styles"],
  devServer: {
    host: "127.0.0.1",
    port: process.env.DEV_PORT_DASHBOARD
      ? parseInt(process.env.DEV_PORT_DASHBOARD)
      : 5175,
  },
});
