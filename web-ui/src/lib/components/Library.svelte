<script lang="ts">
	import { daemonState } from '$lib/stores/daemon_states.svelte';
	import * as playback from '$lib/commands/playback';
	import * as queue from '$lib/commands/queue';
	import * as library from '$lib/commands/library';
	import { getArtUrl } from '$lib/helper';
	import { getHostIp } from '$lib/stores/local_storage.svelte';
	import Track from './Track.svelte';

	let style = $props();
</script>

<div class="flex h-full w-full flex-col space-y-2 overflow-y-auto">
	{#each Object.values(daemonState.library) as track}
		<Track {track} onclick={queue.enqueue(track.id)} />
	{/each}
</div>
