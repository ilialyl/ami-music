import type { Track } from '../types/track';
import { getHostIp } from './stores/local_storage.svelte';

export function getArtUrl(track: Track | null | undefined): string {
	if (!track) return '';
	if (!track.metadata.cover_art_path) return '';
	const filename = track.metadata.cover_art_path.split(/[/\\]/).pop();
	if (!filename) return '';
	return `http://${getHostIp()}:7879/${filename}`;
}
