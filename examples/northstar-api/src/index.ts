export function bearing(from: number, to: number): number {
  return ((to - from) + 360) % 360;
}
