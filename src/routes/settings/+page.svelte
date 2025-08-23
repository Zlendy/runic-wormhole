<script lang="ts">
	import ModeToggle from '$lib/components/mode-toggle.svelte';
	import { Label } from '$lib/components/ui/label';
	import { Input } from '$lib/components/ui/input';
	import { codeLength } from '$lib/localstorage.svelte';
</script>

<div class="grid w-full max-w-sm items-center gap-1.5">
	<ModeToggle />

	<Label for="code_length" class="mt-4">Code Length</Label>
	<Input id="code_length" type="number" bind:value={$codeLength} min={1} max={20} />
</div>
