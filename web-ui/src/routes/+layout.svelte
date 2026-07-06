<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import { onMount } from 'svelte';
	import { connect, connected } from '$lib/ws';
	import { getHostIp } from '$lib/stores/local_storage.svelte';

	let { children } = $props();

	onMount(() => {
		let ip = getHostIp();
		if (ip) {
			connect(ip);
		} else {
			connected.set(false);
		}
	});
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>
{@render children()}
