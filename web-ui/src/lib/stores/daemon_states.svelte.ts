import type { PlayerSnapshot } from '../../types/player_snapshot';
import type { QueueSnapshot } from '../../types/queue_snapshot';
import type { Track } from '../../types/track';
import type { TrackId } from '../../types/track_id';

export interface DaemonStates {
  player: PlayerSnapshot | null;
  queue: QueueSnapshot | null;
  library: { [key: TrackId]: Track };
}

export const daemonState = $state<DaemonStates>({
  player: null,
  queue: null,
  library: {}
});
