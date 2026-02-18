export default defineNuxtConfig({
  extends: '@nuxt/ui',
  modules: ['@pinia/nuxt'],
  ssr: false,
  css: ['~/styles/index.css'],
  devServer: {
    port: process.env.DEV_PORT_SETTINGS ? parseInt(process.env.DEV_PORT_SETTINGS) : 5174,
  },
})
