<script lang="ts">
	import { daemonState } from '$lib/stores/daemon_states.svelte';
	import * as playback from '$lib/commands/playback';
	import * as queue from '$lib/commands/queue';
	import * as library from '$lib/commands/library';
	import TrackButton from './TrackButton.svelte';

	let style = $props();
</script>

<div class="flex h-full w-1/6 shrink-0 flex-col overflow-y-auto">
	{#if daemonState.queue}
		{#each Object.values(daemonState.queue.next_tracks) as track}
			<TrackButton {track} />
		{/each}
	{/if}
</div>
