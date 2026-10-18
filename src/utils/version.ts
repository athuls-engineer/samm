import { api } from '../api/client';

export const STATIC_VERSION: string =
  typeof __APP_VERSION__ !== 'undefined' && __APP_VERSION__
    ? __APP_VERSION__
    : '1.0.0';

let dynamicVersion: string = STATIC_VERSION;

export async function initAppVersion(): Promise<string> {
  try {
    const v = await api.getAppVersion();
    if (v && typeof v === 'string') {
      dynamicVersion = v;
      return v;
    }
  } catch {
    // Fallback to static version
  }
  return dynamicVersion;
}

export function getAppVersion(): string {
  return dynamicVersion;
}
