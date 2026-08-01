import { send } from '../ws';

export function fetch() {
	send({ Library: 'Fetch' });
}
