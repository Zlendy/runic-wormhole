<script lang="ts">
	import SunIcon from '@lucide/svelte/icons/sun';
	import MoonIcon from '@lucide/svelte/icons/moon';
	import SunMoonIcon from '@lucide/svelte/icons/sun-moon';

	import { resetMode, setMode, userPrefersMode } from 'mode-watcher';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
	import { buttonVariants } from '$lib/components/ui/button/index.js';

	const ModeIcon = $derived.by(() => {
		switch (userPrefersMode.current) {
			case 'dark':
				return MoonIcon;
			case 'light':
				return SunIcon;
			default:
				return SunMoonIcon;
		}
	});
</script>

<DropdownMenu.Root>
	<DropdownMenu.Trigger class={buttonVariants({ variant: 'outline', size: 'default' })}>
		<ModeIcon class="h-[1.2rem] w-[1.2rem] scale-100 rotate-0 transition-all" />
		<span class="capitalize">{userPrefersMode.current}</span>
	</DropdownMenu.Trigger>
	<DropdownMenu.Content align="end">
		<DropdownMenu.Item onclick={() => setMode('light')}>Light</DropdownMenu.Item>
		<DropdownMenu.Item onclick={() => setMode('dark')}>Dark</DropdownMenu.Item>
		<DropdownMenu.Item onclick={() => resetMode()}>System</DropdownMenu.Item>
	</DropdownMenu.Content>
</DropdownMenu.Root>
