<script>
	import Library from '$lib/components/Library.svelte';
	import ControlBar from '$lib/components/ControlBar.svelte';
	import Host from '$lib/components/Host.svelte';
	import { connected } from '$lib/ws';
	import RightPanel from '$lib/components/RightPanel.svelte';
</script>

{#if $connected === true}
	<div class="flex h-screen flex-col overflow-hidden dark:bg-black dark:text-white">
		<div class="flex min-h-0 flex-1 flex-row dark:bg-black dark:text-white">
			<div class="w-1/6 shrink-0"></div>
			<Library></Library>
			<RightPanel></RightPanel>
		</div>

		<ControlBar></ControlBar>
	</div>
{:else if $connected === false}
	<div
		class="flex h-screen flex-col items-center justify-center space-y-10 align-middle dark:bg-black dark:text-white"
	>
		<h1 class="text-7xl">Connect</h1>
		<Host></Host>
	</div>
{:else if $connected === null}
	<div
		class="flex h-screen flex-col items-center justify-center space-y-10 align-middle dark:bg-black dark:text-white"
	>
		<h1 class="text-7xl">Loading</h1>
	</div>
{/if}
