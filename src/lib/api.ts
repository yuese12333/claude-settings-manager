import { invoke } from "@tauri-apps/api/core";
import type { Profile, Settings } from "./types";

export function detectSettingsPath() {
  return invoke<string | null>("detect_settings_path");
}

export function loadSettings(path: string) {
  return invoke<Settings>("load_settings", { path });
}

export function loadSettingsRaw(path: string) {
  return invoke<string>("load_settings_raw", { path });
}

export function validateSettingsJson(content: string) {
  return invoke<void>("validate_settings_json", { content });
}

export function saveSettings(path: string, settings: Settings) {
  return invoke<void>("save_settings", { path, settings });
}

export function saveSettingsRaw(path: string, content: string) {
  return invoke<Settings>("save_settings_raw", { path, content });
}

export function pickSettingsFile() {
  return invoke<string | null>("pick_settings_file");
}

export function siblingSettingsPath(path: string) {
  return invoke<string | null>("sibling_settings_path", { path });
}

export function loadProfiles() {
  return invoke<Profile[]>("load_profiles");
}

export function saveProfiles(profiles: Profile[]) {
  return invoke<void>("save_profiles", { profiles });
}
