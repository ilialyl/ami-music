import type { LoopMode } from '../../types/loop_mode';
import type { TrackId } from '../../types/track_id';
import { send } from '../ws';

export function enqueue(track_id: string) {
	send({ Queue: { Enqueue: { track_id } } });
}

export function prepend(track_id: TrackId) {
	send({ Queue: { Prepend: { track_id } } });
}

export function dequeue(index: number) {
	send({ Queue: { Dequeue: { index } } });
}

export function playNow(track_id: TrackId) {
	send({ Queue: { PlayNow: { track_id } } });
}

export function next() {
	send({ Queue: 'Next' });
}

export function prev() {
	send({ Queue: 'Prev' });
}

export function shuffle() {
	send({ Queue: 'Shuffle' });
}

export function clear() {
	send({ Queue: 'Clear' });
}

export function setLoopMode(mode: LoopMode) {
	send({ Queue: { SetLoopMode: mode } });
}

export function cycleLoopMode() {
	send({ Queue: 'CycleLoopMode' });
}

export function fetch() {
	send({ Queue: 'Fetch' });
}
