<script lang="ts">
	import '../app.css';
	import TauriWindow from '$lib/tauri-window.svelte';

	import { Maximize, Minimize, Minus, X } from '@lucide/svelte';
	import { Button } from '$lib/components/ui/button';

	let { children } = $props();
</script>

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

	{@render children()}
</div>
