import type { Track } from "../types/track";

export function get_art_url(track: Track | null | undefined, ip: string): URL | null {
  if (!track) return null;
  if (!track.metadata.cover_art_path) return null;
  const filename = track.metadata.cover_art_path.split(/[/\\]/).pop();
  if (!filename) return null;
    return new URL(`http://${ip}:7879/${filename}`);
}
