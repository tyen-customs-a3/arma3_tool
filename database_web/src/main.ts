import './style.css';
import { GraphQueryParams } from './modules/types';
import { WebSocketService } from './modules/websocket';
import { UIManager } from './modules/uiManager';
import { CosmographManager } from './lib/graph';
import { createGraphConfig } from './lib/graph/config';

// Initialize managers
const graphContainer = document.getElementById('graph') as HTMLDivElement;
const uiManager = new UIManager();
const wsService = new WebSocketService('ws://localhost:3000/ws');

// Get control elements
const minDistanceInput = document.getElementById('min-distance') as HTMLInputElement;
const pointSizeInput = document.getElementById('point-size') as HTMLInputElement;
const linkWidthInput = document.getElementById('link-width') as HTMLInputElement;
const curvedLinesInput = document.getElementById('curved-lines') as HTMLInputElement;
const spaceSizeInput = document.getElementById('space-size') as HTMLInputElement;

// Get simulation control elements
const disableSimulationInput = document.getElementById('disable-simulation') as HTMLInputElement;
const gravityInput = document.getElementById('gravity') as HTMLInputElement;
const repulsionInput = document.getElementById('repulsion') as HTMLInputElement;
const linkSpringInput = document.getElementById('link-spring') as HTMLInputElement;
const decayInput = document.getElementById('decay') as HTMLInputElement;

// Get source filter elements
const excludePatternsInput = document.getElementById('exclude-patterns') as HTMLInputElement;
const impactClassesInput = document.getElementById('impact-classes') as HTMLInputElement;
const orphanedNodesList = document.getElementById('orphaned-nodes-list') as HTMLDivElement;

// Get or create label control elements
let showLabelsInput = document.getElementById('show-labels') as HTMLInputElement;
if (!showLabelsInput) {
  // Create the label control if it doesn't exist
  const actionsDiv = document.querySelector('.actions') as HTMLDivElement;
  if (actionsDiv) {
    const labelSection = document.createElement('div');
    labelSection.innerHTML = `
      <div class="actions-header">Label Settings</div>
      <div class="checkbox-group">
        <label for="show-labels">Show Labels</label>
        <input type="checkbox" id="show-labels" checked>
      </div>
    `;
    actionsDiv.appendChild(labelSection);
    showLabelsInput = document.getElementById('show-labels') as HTMLInputElement;
  }
}

// Initialize UI with default values
function initializeUI() {
  // Set default values for UI controls
  if (minDistanceInput) minDistanceInput.value = '50';
  if (pointSizeInput) pointSizeInput.value = '5';
  if (linkWidthInput) linkWidthInput.value = '1';
  if (curvedLinesInput) curvedLinesInput.checked = true;
  if (spaceSizeInput) spaceSizeInput.value = '8192';
  if (disableSimulationInput) disableSimulationInput.checked = false;
  if (gravityInput) gravityInput.value = '0.1';
  if (repulsionInput) repulsionInput.value = '0.5';
  if (linkSpringInput) linkSpringInput.value = '0.1';
  if (decayInput) decayInput.value = '100';
  if (showLabelsInput) showLabelsInput.checked = true;
}

// Function to get current graph config
function getConfig() {
  const spaceSize = spaceSizeInput ? parseInt(spaceSizeInput.value) : 1000;
  return createGraphConfig(
    (index: number | undefined) => {
      if (index !== undefined) {
        const node = cosmographManager.getNodeByIndex(index);
        if (node) {
          cosmographManager.focusNode(node);
          const childNodes = cosmographManager.findChildNodes(node.id);
          uiManager.updateNodeDetails({
            name: node.name || '-',
            id: node.id,
            depth: node.depth?.toString() || '-',
            children: childNodes.length.toString(),
            parent: node.parent_id || '-',
            container: node.container_class || '-',
            source: node.source_path || '-'
          });
        }
      } else {
        cosmographManager.clearFocus();
        uiManager.clearNodeDetails();
      }
    },
    spaceSize
  );
}

// Initialize graph manager with the container and initial config
const cosmographManager = new CosmographManager({ 
  container: graphContainer, 
  config: createGraphConfig(undefined, spaceSizeInput ? parseInt(spaceSizeInput.value) : 1000)
});

// Initialize UI with default values
initializeUI();

// Function to get source filter patterns
function getSourceFilters(): { excludeSourcePatterns: string[] } {
  const excludeInput = document.getElementById('exclude-patterns') as HTMLInputElement;
  const excludePatterns = excludeInput.value
    .split(',')
    .map(p => p.trim())
    .filter(p => p.length > 0);

  return {
    excludeSourcePatterns: excludePatterns
  };
}

// Function to update orphaned nodes list
function updateOrphanedNodesList(orphanedClasses: string[]) {
  orphanedNodesList.innerHTML = '';
  orphanedClasses.forEach((className: string) => {
    const div = document.createElement('div');
    div.className = 'orphaned-node-item';
    div.textContent = className;
    div.addEventListener('click', () => {
      cosmographManager.selectNodeById(className);
    });
    orphanedNodesList.appendChild(div);
  });
}

// Function to request impact analysis
async function requestImpactAnalysis() {
  const classesToRemove = impactClassesInput.value
    .split(',')
    .map(c => c.trim())
    .filter(c => c.length > 0);

  if (classesToRemove.length === 0) {
    console.warn('No classes specified for impact analysis');
    return;
  }

  const filters = getSourceFilters();
  try {
    await wsService.requestImpactAnalysis({
      classesToRemove,
      excludeSourcePatterns: filters.excludeSourcePatterns
    });
  } catch (error) {
    console.error('Error requesting impact analysis:', error);
  }
}

// Setup WebSocket handlers
wsService.on('graph_data', async (data) => {
  console.log('Received graph data:', data);
  await cosmographManager.updateGraph(data);
});

wsService.on('database_response', async (data) => {
  console.log('Received database response:', data);
  if (data.success && data.data) {
    const result = data.data;
    if (result.graphData) {
      // Update graph with impact analysis data
      await cosmographManager.updateGraph(result.graphData);
      // Update orphaned nodes list
      updateOrphanedNodesList(result.orphanedClasses);
    }
  }
});

// Request initial data after setting up handlers
const initialFilters = getSourceFilters();
void wsService.requestFullGraphData({
  excludeSourcePatterns: initialFilters.excludeSourcePatterns,
  maxDepth: 100
}).catch(error => {
  console.error('Error requesting initial graph data:', error);
});

// Function to request graph data
async function requestGraphData() {
  const filters = getSourceFilters();
  const params: GraphQueryParams = {
    excludeSourcePatterns: filters.excludeSourcePatterns,
    maxDepth: 100
  };
  try {
    await wsService.requestFullGraphData(params);
  } catch (error) {
    console.error('Error requesting graph data:', error);
  }
}

// Add event listeners for UI controls
minDistanceInput?.addEventListener('change', async () => {
  await cosmographManager.setConfig(getConfig());
  if (!disableSimulationInput.checked) {
    await cosmographManager.restart();
  }
});

pointSizeInput?.addEventListener('change', async () => {
  await cosmographManager.setConfig(getConfig());
});

linkWidthInput?.addEventListener('change', async () => {
  await cosmographManager.setConfig(getConfig());
});

curvedLinesInput?.addEventListener('change', async () => {
  await cosmographManager.setConfig(getConfig());
});

spaceSizeInput?.addEventListener('change', async () => {
  await cosmographManager.setConfig(getConfig());
});

// Add event listener for label checkbox
if (showLabelsInput) {
  showLabelsInput.addEventListener('change', () => {
    cosmographManager.toggleLabels(showLabelsInput.checked);
  });
}

// Add event listeners for simulation controls
disableSimulationInput?.addEventListener('change', async () => {
  await cosmographManager.setConfig(getConfig());
  if (disableSimulationInput.checked) {
    cosmographManager.pause();
  } else {
    cosmographManager.start();
  }
});

gravityInput?.addEventListener('change', async () => {
  await cosmographManager.setConfig(getConfig());
  if (!disableSimulationInput.checked) {
    await cosmographManager.restart();
  }
});

repulsionInput?.addEventListener('change', async () => {
  await cosmographManager.setConfig(getConfig());
  if (!disableSimulationInput.checked) {
    await cosmographManager.restart();
  }
});

linkSpringInput?.addEventListener('change', async () => {
  await cosmographManager.setConfig(getConfig());
  if (!disableSimulationInput.checked) {
    await cosmographManager.restart();
  }
});

decayInput?.addEventListener('change', async () => {
  await cosmographManager.setConfig(getConfig());
  if (!disableSimulationInput.checked) {
    await cosmographManager.restart();
  }
});

// Add event listeners for source filters
excludePatternsInput?.addEventListener('change', () => {
  void requestGraphData();
});

// Add event listeners for buttons
document.getElementById('regenerate')?.addEventListener('click', () => {
  void requestGraphData();
});

// Add event listener for impact analysis button
document.getElementById('analyze-impact')?.addEventListener('click', () => {
  void requestImpactAnalysis();
});

// Add event listeners for buttons
document.getElementById('fit-view')?.addEventListener('click', () => cosmographManager.fitView());
document.getElementById('select-point')?.addEventListener('click', () => {
  const randomIndex = cosmographManager.getRandomPointIndex();
  const randomNode = cosmographManager.getNodeByIndex(randomIndex);
  if (randomNode) {
    cosmographManager.focusNode(randomNode);
    const childNodes = cosmographManager.findChildNodes(randomNode.id);
    uiManager.updateNodeDetails({
      name: randomNode.name || '-',
      id: randomNode.id,
      depth: randomNode.depth?.toString() || '-',
      children: childNodes.length.toString(),
      parent: randomNode.parent_id || '-',
      container: randomNode.container_class || '-',
      source: randomNode.source_path || '-'
    });
  }
});
