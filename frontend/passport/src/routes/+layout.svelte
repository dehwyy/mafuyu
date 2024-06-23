<script lang="ts">
  import '../app.postcss'
  import 'highlight.js/styles/github-dark.css'

  import {
    arrow,
    autoUpdate,
    computePosition,
    flip,
    offset,
    shift
  } from '@floating-ui/dom'
  import { ProgressBar } from '@prgm/sveltekit-progress-bar'
  import {
    AppShell,
    getModalStore,
    initializeStores,
    Modal,
    storeHighlightJs,
    storePopup,
    Toast
  } from '@skeletonlabs/skeleton'
  // import hljs from "highlight.js/lib/core"
  // import xml from "highlight.js/lib/languages/xml"
  // import css from "highlight.js/lib/languages/css"
  // import javascript from "highlight.js/lib/languages/javascript"
  // import typescript from "highlight.js/lib/languages/typescript"
  import { hydrate, QueryClientProvider } from '@tanstack/svelte-query'
  import { SvelteToast } from '@zerodevx/svelte-toast'
  // Well, it should exist.
  // @ts-ignore
  import { afterNavigate, onNavigate } from '$app/navigation'
  import Header from '$lib/components/header/header.svelte'
  import StarsBackground from '$lib/components/hoshisora.svelte'
  import { queryClient } from '$lib/query-client'
  import { authedUserStore } from '$lib/stores/user'
  import type { AfterNavigate } from '@sveltejs/kit'

  // skeleton stores
  initializeStores()
  // Highlight JS
  // hljs.registerLanguage("xml", xml) // html
  // hljs.registerLanguage("css", css)
  // hljs.registerLanguage("javascript", javascript)
  // hljs.registerLanguage("typescript", typescript)
  // storeHighlightJs.set(hljs)
  // Floating UI for Popups
  storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow })

  // Scroll to top onNavigation
  afterNavigate((params: AfterNavigate) => {
    const isNewPage = params.from?.url.pathname !== params.to?.url.pathname
    const pageElement = document.querySelector('#page')
    if (isNewPage && pageElement) {
      pageElement.scrollTop = 0
    }
  })

  // Perform transition (Chromium only, if I'm not mistaking)
  // @ts-ignore
  onNavigate((navigation) => {
    // @ts-ignore
    if (!document.startViewTransition) return

    return new Promise((resolve) => {
      // @ts-ignore
      document.startViewTransition(async () => {
        // @ts-ignore
        resolve()
        await navigation.complete
      })
    })
  })

  // hide scroll when modal
  const modal_store = getModalStore()
  modal_store.subscribe((v) => {
    if (typeof window === 'undefined') return

    let body = document.querySelector('body')
    body!.style.overflow = v.length ? 'hidden' : 'auto'
  })

  export let data: import('./$types').LayoutData
  hydrate(queryClient, data.dehydrateState)

  authedUserStore.set(
    data.username
      ? {
          id: data.userId,
          username: data.username
        }
      : null
  )
</script>

<ProgressBar
  class="text-primary-600"
  zIndex={100}
  settleTime={100}
/>
<QueryClientProvider client={queryClient}>
  <Modal />
  <StarsBackground>
    <AppShell>
      <svelte:fragment slot="header">
        <Header />
      </svelte:fragment>
      <SvelteToast />
      <Toast />
      <slot />
    </AppShell>
  </StarsBackground>
</QueryClientProvider>
