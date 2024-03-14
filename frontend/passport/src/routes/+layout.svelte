<script lang="ts">
  import "../app.postcss"
  import "highlight.js/styles/github-dark.css"
  import { initializeStores, Modal, Toast, getModalStore, AppShell, storeHighlightJs, storePopup } from "@skeletonlabs/skeleton"
  import hljs from "highlight.js/lib/core"
  import { computePosition, autoUpdate, flip, shift, offset, arrow } from "@floating-ui/dom"
  import xml from "highlight.js/lib/languages/xml"
  import css from "highlight.js/lib/languages/css"
  import javascript from "highlight.js/lib/languages/javascript"
  import typescript from "highlight.js/lib/languages/typescript"
  import { hydrate, QueryClientProvider } from "@tanstack/svelte-query"
  import type { AfterNavigate } from "@sveltejs/kit"
  import { afterNavigate, onNavigate } from "$app/navigation"
  import { SvelteToast } from "@zerodevx/svelte-toast"
  import { queryClient } from "$lib/query-client"

  // skeleton stores
  initializeStores()
  // Highlight JS
  hljs.registerLanguage("xml", xml) // html
  hljs.registerLanguage("css", css)
  hljs.registerLanguage("javascript", javascript)
  hljs.registerLanguage("typescript", typescript)
  storeHighlightJs.set(hljs)
  // Floating UI for Popups
  storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow })

  // Scroll to top onNavigation
  afterNavigate((params: AfterNavigate) => {
    const is_new_page = params.from?.url.pathname !== params.to?.url.pathname
    const page_element = document.querySelector("#page")
    if (is_new_page && page_element) {
      page_element.scrollTop = 0
    }
  })

  // Perform transition (Chromium only, if I'm not mistaking)
  onNavigate(navigation => {
    // @ts-ignore
    if (!document.startViewTransition) return

    return new Promise(resolve => {
      // @ts-ignore
      document.startViewTransition(async () => {
        resolve()
        await navigation.complete
      })
    })
  })

  // hide scroll when modal
  const modal_store = getModalStore()
  modal_store.subscribe(v => {
    if (typeof window === "undefined") return

    let body = document.querySelector("body")
    body!.style.overflow = v.length ? "hidden" : "auto"
  })

  // Before this line, everything is INIT
  import Header from "$lib/components/header/header.svelte"
  import { ProgressBar } from "@prgm/sveltekit-progress-bar"
  import { authedUserStore } from "$lib/stores/user"

  export let data: import("./$types").LayoutData
  hydrate(queryClient, data.dehydrateState)

  authedUserStore.set(
    data.username
      ? {
          id: data.userId,
          username: data.username,
        }
      : null,
  )
</script>

<ProgressBar class="text-primary-600" zIndex={100} settleTime={100} />
<QueryClientProvider client={queryClient}>
  <Modal />
  <AppShell>
    <svelte:fragment slot="header">
      <Header />
    </svelte:fragment>
    <SvelteToast />
    <Toast />
    <slot />
  </AppShell>
</QueryClientProvider>
