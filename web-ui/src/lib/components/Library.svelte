<script lang="ts">
	import { daemonState } from '$lib/stores/daemon_states.svelte';
	import * as playback from '$lib/commands/playback';
	import * as queue from '$lib/commands/queue';
	import * as library from '$lib/commands/library';
	import { getArtUrl } from '$lib/helper';
	import { getHostIp } from '$lib/stores/local_storage.svelte';
	import type { Track } from '../../types/track';

	let style = $props();
</script>

<div class="flex h-full w-full flex-col space-y-2 overflow-y-auto">
	{#each Object.values(daemonState.library) as track}
		<button
			class="track flex w-full cursor-pointer flex-row items-stretch space-x-3 text-left text-sm hover:bg-black hover:text-white"
			onclick={() => queue.enqueue(track.id)}
		>
			<div class="relative aspect-square shrink-0">
				<div class="relative aspect-square shrink-0 bg-white">
					{#if getArtUrl(track)}
						<img
							class="absolute inset-0 h-full w-full object-cover object-center"
							src={getArtUrl(track)}
							alt=""
						/>
					{/if}
				</div>
			</div>
			<div>
				{track.metadata.title}<br />{track.metadata.artist ?? 'Unknown'}
			</div>
		</button>
	{/each}
</div>
