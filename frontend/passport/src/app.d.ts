// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
// and what to do when importing types
declare namespace App {
	// interface Locals {}
	// interface PageData {}
	// interface Error {}
	// interface Platform {}
}

declare module 'svelte-icons/*' {
	import { SvelteComponent } from 'svelte'
	export default class SvelteIcon extends SvelteComponent {}
}
