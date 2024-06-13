export function parseJSON(json: string): any {
  try {
    return JSON.parse(json);
  } catch (_) {
    return null;
  }
}
