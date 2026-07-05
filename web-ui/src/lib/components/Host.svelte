<script lang="ts">
	import * as playback from '$lib/commands/playback';
	import * as queue from '$lib/commands/queue';
	import * as library from '$lib/commands/library';
	import { connect } from '$lib/ws';

	let value = $state('');

	function onkeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') {
			connect(value);
		}
	}

	
</script>

<input
	type="string"
	class="rounded-lg border-2 border-black dark:border-white"
	bind:value
	placeholder="Enter Host's IP address"
	{onkeydown}
/>
