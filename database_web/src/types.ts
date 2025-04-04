export interface Node {
  id: string;
  label?: string;
  color?: string;
}

export interface Link {
  source: string;
  target: string;
  color?: string | [number, number, number, number];
}
