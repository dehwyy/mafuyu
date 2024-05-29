// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },
  modules: ["nuxt-primevue"],
  css: ["~/assets/css/main.css", "primevue/resources/themes/aura-dark-pink/theme.css"],
  primevue: {
    options: {
      ripple: true,
    },
  },
})
