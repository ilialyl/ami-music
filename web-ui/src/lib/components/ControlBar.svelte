<script lang="ts">
	import * as playback from '$lib/commands/playback';
	import * as queue from '$lib/commands/queue';
	import * as library from '$lib/commands/library';
	import { daemonState } from '$lib/stores/daemon_states.svelte';

	let currentTrack = $derived(daemonState.queue?.current_track);
</script>

<div
	class="flex h-1/12 shrink-0 flex-row items-center justify-between space-x-5 border-t-2 bg-black px-10 text-white"
>
	<div>
		<button class="cursor-pointer" onclick={playback.play} aria-label="Play"
			><svg
				xmlns="http://www.w3.org/2000/svg"
				fill="none"
				viewBox="0 0 24 24"
				stroke-width="1.5"
				stroke="currentColor"
				class="size-6"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					d="M5.25 5.653c0-.856.917-1.398 1.667-.986l11.54 6.347a1.125 1.125 0 0 1 0 1.972l-11.54 6.347a1.125 1.125 0 0 1-1.667-.986V5.653Z"
				/>
			</svg>
		</button>
		<button class="cursor-pointer" onclick={playback.pause} aria-label="Pause"
			><svg
				xmlns="http://www.w3.org/2000/svg"
				fill="none"
				viewBox="0 0 24 24"
				stroke-width="1.5"
				stroke="currentColor"
				class="size-6"
			>
				<path stroke-linecap="round" stroke-linejoin="round" d="M15.75 5.25v13.5m-7.5-13.5v13.5" />
			</svg>
		</button>
	</div>
	<div>
		<h1 class="text-lg font-bold">
			{#if currentTrack}
				{currentTrack.metadata.title} - {currentTrack.metadata.artist}
			{/if}
		</h1>
	</div>
</div>
