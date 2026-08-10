import type { Track } from '../types/track';
import { getHostIp } from './stores/local_storage.svelte';
import { artPort } from './ws';

export function getArtUrl(track: Track | null | undefined): string {
	if (!track) return '';
	if (!track.metadata.cover_art_path) return '';
	const filename = track.metadata.cover_art_path.split(/[/\\]/).pop();
	if (!filename) return '';
	return `http://${getHostIp()}:${artPort}/${filename}`;
}
