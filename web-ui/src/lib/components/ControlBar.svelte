<script lang="ts">
	import * as playback from '$lib/commands/playback';
	import * as queue from '$lib/commands/queue';
	import * as library from '$lib/commands/library';
	import { daemonState } from '$lib/stores/daemon_states.svelte';
	import { Pause, Play, SkipBack, SkipForward } from '@lucide/svelte';
	import { getHostIp } from '$lib/stores/local_storage.svelte';
	import { getArtUrl } from '$lib/helper';

	let currentTrack = $derived(daemonState.queue?.current_track);

	let time = $derived(
		daemonState.player?.position.secs != null
			? `${Math.floor(daemonState.player.position.secs / 60)}:${(daemonState.player.position.secs % 60).toString().padStart(2, '0')}`
			: '0:00'
	);
</script>

<div
	class="flex h-1/12 shrink-0 flex-row items-center justify-between space-x-5 border-t-2 bg-black text-white"
>
	<div>
		{time}
	</div>
	<div class="flex space-x-5">
		<button class="cursor-pointer" onclick={queue.prev}>
			<SkipBack strokeWidth={1} />
		</button>

		{#if daemonState.player?.playback_status === 'Paused'}
			<button class="cursor-pointer" onclick={playback.play}>
				<Play strokeWidth={1} />
			</button>
		{:else}
			<button class="cursor-pointer" onclick={playback.pause}>
				<Pause strokeWidth={1} />
			</button>
		{/if}
		<button class="cursor-pointer" onclick={queue.next}>
			<SkipForward strokeWidth={1} />
		</button>
	</div>
	<div class="flex h-full items-center space-x-5">
		{#if currentTrack}
			<h1 class="text-lg font-bold">
				{currentTrack.metadata.title} - {currentTrack.metadata.artist}
			</h1>

			<img class="h-full" src={getArtUrl(currentTrack)} alt="cover" />
		{/if}
	</div>
</div>
