<script lang="ts">
	import '../app.css';
	import { Button } from '$lib/components/ui/button';
	import { wormhole } from '$lib/wormhole.svelte';
	import { ModeWatcher } from 'mode-watcher';

	interface Route {
		href: string;
		name: string;
	}

	const routes: Route[] = [
		{
			href: 'send',
			name: 'Send'
		},
		{
			href: 'receive',
			name: 'Receive'
		},
		{
			href: 'settings',
			name: 'Settings'
		}
	];

	let { children } = $props();
</script>

{#snippet nav()}
	{#each routes as route}
		<Button class="flex-grow min-w-26" href={route.href} disabled={wormhole.active !== null && wormhole.active !== route.href}>
			{route.name}
		</Button>
	{/each}
{/snippet}

<ModeWatcher />

<header class="mt-4 hidden flex-wrap justify-center gap-4 sm:flex max-w-xl mx-auto px-4">
	{@render nav()}
</header>

<main class="flex justify-center p-4">
	{@render children()}
</main>

<footer class="fixed right-0 bottom-4 left-0 flex flex-wrap justify-center gap-4 sm:hidden mx-auto px-4">
	{@render nav()}
</footer>
