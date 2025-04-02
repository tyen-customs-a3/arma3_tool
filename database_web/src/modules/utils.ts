export function getRandomInRange([min, max]: [number, number]): number {
  return Math.random() * (max - min) + min;
} 