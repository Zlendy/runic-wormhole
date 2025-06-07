<script lang="ts">
	import '../app.css';
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

<header class="mt-4 hidden flex-wrap justify-center gap-4 sm:flex">
	{@render nav()}
</header>

<main class="flex justify-center p-4">
	{@render children()}
</main>

<footer class="fixed right-0 bottom-4 left-0 flex flex-wrap justify-center gap-4 sm:hidden">
	{@render nav()}
</footer>
