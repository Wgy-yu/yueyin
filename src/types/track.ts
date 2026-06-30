export type SourceType = "netease" | "qq" | "local";
export type PlayMode = "loop" | "shuffle" | "single";

export interface Track {
  id: string;
  name: string;
  artist: string;
  album?: string;
  coverUrl?: string;
  duration?: number;
  source: SourceType;
  // ponytail: extra fields stored as opaque JSON for provider-specific data
  extra?: Record<string, unknown>;
}
