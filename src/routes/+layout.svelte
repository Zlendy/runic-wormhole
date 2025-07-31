<script lang="ts">
	import '../app.css';
	import { Button } from '$lib/components/ui/button';
	import { wormhole } from '$lib/wormhole.svelte';
	import { ModeWatcher } from 'mode-watcher';
	import { Download, Icon, Send, Settings } from '@lucide/svelte';

	interface Route {
		icon: typeof Icon;
		href: string;
		name: string;
	}

	const routes: Route[] = [
		{
			icon: Send,
			href: 'send',
			name: 'Send'
		},
		{
			icon: Download,
			href: 'receive',
			name: 'Receive'
		},
		{
			icon: Settings,
			href: 'settings',
			name: 'Settings'
		}
	];

	let { children } = $props();
</script>

{#snippet nav()}
	{#each routes as route}
		<Button
			class="min-w-26 flex-grow"
			href={route.href}
			disabled={wormhole.active !== null && wormhole.active !== route.href}
		>
			<route.icon />
			{route.name}
		</Button>
	{/each}
{/snippet}

<ModeWatcher />

<header class="mx-auto mt-4 hidden max-w-xl flex-wrap justify-center gap-4 px-4 sm:flex">
	{@render nav()}
</header>

<main class="flex justify-center p-4">
	{@render children()}
</main>

<footer
	class="fixed right-0 bottom-4 left-0 mx-auto flex flex-wrap justify-center gap-4 px-4 sm:hidden"
>
	{@render nav()}
</footer>
