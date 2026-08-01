import { send } from '../ws';

export function play() {
	send({ Playback: 'Play' });
}

export function pause() {
	send({ Playback: 'Pause' });
}

export function toggle_play() {
	send({ Playback: 'TogglePlay' });
}

export function set_position(position: { secs: number; nanos: number }) {
	send({ Playback: { SetPosition: { position } } });
}

export function seek(offset_seconds: bigint) {
	send({ Playback: { Seek: { offset_seconds } } });
}

export function restart() {
	send({ Playback: 'Restart' });
}

export function increase_vol(step: number) {
	send({ Playback: { IncreaseVol: { step } } });
}

export function decrease_vol(step: number) {
	send({ Playback: { DecreaseVol: { step } } });
}

export function set_volume(value: number) {
	send({ Playback: { SetVolume: { value } } });
}

export function fetch() {
	send({ Playback: 'GetSnapshot' });
}
