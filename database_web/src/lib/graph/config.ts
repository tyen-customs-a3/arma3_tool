import { CosmographConfig } from '@cosmograph/cosmograph';

// Default configuration values
const defaultConfig: CosmographConfig = {
  // Graph settings
  pointColor: '#FFFFFF',
  linkColor: '#666666',
  backgroundColor: '#000000',
  curvedLinks: true,
  spaceSize: 8192,
  // Label settings
  showLabels: true,
  pointLabelBy: 'name',
  // Interactivity settings
  onClick: undefined
};

/**
 * Creates a Cosmograph configuration with the given settings
 */
export function createGraphConfig(
  onNodeClick?: (index: number | undefined, pointPosition: [number, number] | undefined, event: MouseEvent) => void,
  spaceSize?: number
): CosmographConfig {
  return {
    ...defaultConfig,
    spaceSize: spaceSize ?? defaultConfig.spaceSize,
    onClick: onNodeClick
  };
}

export { defaultConfig }; 