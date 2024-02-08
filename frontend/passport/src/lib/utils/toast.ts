import { toast } from "@zerodevx/svelte-toast"
import Check from "$lib/assets/check.svg?raw"

const with_icon = (icon: string, html: string) => {
  return `<section style="display:flex;gap:0 1rem;align-items:center;word-break:break-word"><div style="width:32px;font-size:26px">${icon}</div><div>${html}</div></section>`
}

export class Toasts {
  static success(html: string) {
    toast.push(with_icon(Check, html), {
      theme: {
        "--toastBackground": "rgba(var(--color-success-500))",
        "--toastBarBackground": "rgba(var(--color-success-800))",
      },
    })
  }
  static default(html: string) {
    toast.push(with_icon("❄️", html), {
      theme: {
        "--toastBackground": "rgba(var(--color-surface-600))",
        "--toastBarBackground": "rgba(var(--color-surface-200))",
      },
    })
  }

  static error(html: string) {
    toast.push(html, {
      theme: {
        "--toastBackground": "rgba(var(--color-error-500))",
        "--toastBarBackground": "rgba(var(--color-error-800))",
      },
    })
  }
}
