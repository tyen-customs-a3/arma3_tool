import { GraphConfig } from './types';

export interface SimulationConfig {
  // Node settings
  nodeCount: number;
  minDistance: number;
  pointSize: number;
  linkWidth: number;
  curvedLines: boolean;

  // Simulation settings
  disableSimulation: boolean;
  gravity: number;
  repulsion: number;
  linkSpring: number;
  decay: number;

  // Graph settings
  spaceSize: number;
  backgroundColor: string;
  pointColor: string;
  linkColor: string;
  hoveredPointRingColor: string;
}

export const defaultConfig: SimulationConfig = {
  // Node settings
  nodeCount: 1000,
  minDistance: 1,
  pointSize: 4,
  linkWidth: 0.1,
  curvedLines: true,

  // Simulation settings
  disableSimulation: false,
  gravity: 0.05,
  repulsion: 0.5,
  linkSpring: 0.5,
  decay: 10000,

  // Graph settings
  spaceSize: 4096,
  backgroundColor: '#151515',
  pointColor: '#5F74C2',
  linkColor: '#5F74C2',
  hoveredPointRingColor: '#FFFFFF'
};

export function createGraphConfig(simConfig: SimulationConfig, onClick: (node: any, index: number | undefined) => void): GraphConfig {
  return {
    simulation: {
      repulsion: simConfig.repulsion,
      gravity: simConfig.gravity,
      linkSpring: simConfig.linkSpring,
      linkDistance: simConfig.minDistance,
      decay: simConfig.decay,
      disabled: simConfig.disableSimulation
    },
    renderLinks: true,
    linkWidth: simConfig.linkWidth,
    linkColor: simConfig.linkColor,
    nodeColor: simConfig.pointColor,
    nodeSize: simConfig.pointSize,
    backgroundColor: simConfig.backgroundColor,
    curvedLinks: simConfig.curvedLines,
    spaceSize: simConfig.spaceSize,
    events: {
      onClick: (node, index) => onClick(node, index)
    }
  };
}