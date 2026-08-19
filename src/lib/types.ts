export type Settings = {
  env?: Record<string, string>;
  enabledPlugins?: Record<string, boolean>;
  extraKnownMarketplaces?: Record<string, unknown>;
  theme?: string | null;
  [key: string]: unknown;
};

export type Profile = {
  id: string;
  name: string;
  baseUrl: string;
  apiKey: string;
};

export const MODELS = [
  "claude-opus-4-7",
  "claude-sonnet-4-6",
  "claude-haiku-4-5-20251001",
] as const;
