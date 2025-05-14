<script lang="ts">
  import { isMaximized } from '$lib/window.svelte';
	import '../app.css';
	let { children } = $props();
</script>

<div class="bg-background h-screen" class:rounded-lg={!isMaximized.value}>
	{@render children()}
</div>
