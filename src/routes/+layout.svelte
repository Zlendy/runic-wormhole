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

<header class="hidden flex-wrap justify-center gap-4 sm:flex mt-4">
	{@render nav()}
</header>

<main class="flex justify-center p-4">
	{@render children()}
</main>

<footer class="fixed bottom-4 left-0 right-0 flex flex-wrap justify-center gap-4 sm:hidden">
	{@render nav()}
</footer>
