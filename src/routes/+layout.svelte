<script lang="ts">
	import '../app.css';
	import TauriWindow from '$lib/tauri-window.svelte';

	import { Maximize, Minimize, Minus, X } from '@lucide/svelte';
	import { Button } from '$lib/components/ui/button';

	interface Route {
		href: string;
		name: string;
	}

	const routes: Route[] = [
		{
			href: '/send',
			name: 'Send'
		},
		{
			href: '/receive',
			name: 'Receive'
		},
		{
			href: '/settings',
			name: 'Settings'
		}
	];

	let { children } = $props();
</script>

{#snippet nav()}
	{#each routes as route}
		<Button href={route.href}>
			{route.name}
		</Button>
	{/each}
{/snippet}

<div class="h-screen bg-background" class:rounded-lg={!TauriWindow.isMaximized}>
	<div class="mb-4 flex flex-row-reverse gap-1 border-b p-1" data-tauri-drag-region>
		<Button
			class="focus:bg-accent focus:text-accent-foreground focus-visible:ring-0 focus-visible:ring-offset-0"
			variant="ghost"
			size="icon"
			onclick={TauriWindow.close}
		>
			<X />
		</Button>
		<Button
			class="focus:bg-accent focus:text-accent-foreground focus-visible:ring-0 focus-visible:ring-offset-0"
			variant="ghost"
			size="icon"
			onclick={TauriWindow.toggleMaximize}
		>
			{#if TauriWindow.isMaximized}
				<Minimize />
			{:else}
				<Maximize />
			{/if}
		</Button>
		<Button
			class="focus:bg-accent focus:text-accent-foreground focus-visible:ring-0 focus-visible:ring-offset-0"
			variant="ghost"
			size="icon"
			onclick={TauriWindow.minimize}
		>
			<Minus />
		</Button>
	</div>

	<header class="hidden flex-wrap justify-center gap-4 sm:flex">
		{@render nav()}
	</header>

	<main class="flex justify-center p-4">
		{@render children()}
	</main>

	<footer class="fixed bottom-4 left-0 right-0 flex flex-wrap justify-center gap-4 sm:hidden">
		{@render nav()}
	</footer>
</div>
