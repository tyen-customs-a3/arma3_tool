import './style.css';
import { GraphConfig } from './modules/types';
import { WebSocketService } from './modules/websocket';
import { GraphManager } from './modules/graphManager';
import { UIManager } from './modules/uiManager';
import { getRandomInRange } from './modules/utils';
import { SimulationConfig, defaultConfig, createGraphConfig } from './modules/config';

// Initialize managers
const div = document.getElementById('graph') as HTMLDivElement;
const uiManager = new UIManager(togglePause, restartSimulation);
const wsService = new WebSocketService('ws://localhost:3000/ws');

// Get control elements
const nodeCountInput = document.getElementById('node-count') as HTMLInputElement;
const minDistanceInput = document.getElementById('min-distance') as HTMLInputElement;
const pointSizeInput = document.getElementById('point-size') as HTMLInputElement;
const linkWidthInput = document.getElementById('link-width') as HTMLInputElement;
const curvedLinesInput = document.getElementById('curved-lines') as HTMLInputElement;
const spaceSizeInput = document.getElementById('space-size') as HTMLInputElement;

// Get simulation control elements
const gravityInput = document.getElementById('gravity') as HTMLInputElement;
const repulsionInput = document.getElementById('repulsion') as HTMLInputElement;
const linkSpringInput = document.getElementById('link-spring') as HTMLInputElement;
const decayInput = document.getElementById('decay') as HTMLInputElement;
const disableSimulationInput = document.getElementById('disable-simulation') as HTMLInputElement;

// Initialize UI with default values
function initializeUI(config: SimulationConfig) {
  nodeCountInput.value = config.nodeCount.toString();
  minDistanceInput.value = config.minDistance.toString();
  pointSizeInput.value = config.pointSize.toString();
  linkWidthInput.value = config.linkWidth.toString();
  curvedLinesInput.checked = config.curvedLines;
  spaceSizeInput.value = config.spaceSize.toString();
  gravityInput.value = config.gravity.toString();
  repulsionInput.value = config.repulsion.toString();
  linkSpringInput.value = config.linkSpring.toString();
  decayInput.value = config.decay.toString();
  disableSimulationInput.checked = config.disableSimulation;
}

// Function to get current config from UI
function getCurrentSimConfig(): SimulationConfig {
  return {
    ...defaultConfig,
    nodeCount: parseInt(nodeCountInput.value),
    minDistance: parseFloat(minDistanceInput.value),
    pointSize: parseFloat(pointSizeInput.value),
    linkWidth: parseFloat(linkWidthInput.value),
    curvedLines: curvedLinesInput.checked,
    spaceSize: parseInt(spaceSizeInput.value),
    disableSimulation: disableSimulationInput.checked,
    gravity: parseFloat(gravityInput.value),
    repulsion: parseFloat(repulsionInput.value),
    linkSpring: parseFloat(linkSpringInput.value),
    decay: parseFloat(decayInput.value)
  };
}

// Function to get current graph config
function getConfig(): GraphConfig {
  return createGraphConfig(getCurrentSimConfig(), (node, index) => {
    if (node && index !== undefined) {
      graphManager.selectNodeById(node.id);
      graphManager.zoomToNodeById(node.id);
      const childNodes = graphManager.findChildNodes(node.id);
      uiManager.updateNodeDetails({
        name: node.name || '-',
        id: node.id,
        depth: node.depth?.toString() || '-',
        children: childNodes.length.toString()
      });
    } else {
      uiManager.clearNodeDetails();
    }
  });
}

// Initialize graph manager
const graphManager = new GraphManager(div, getConfig());

// Initialize UI with default values
initializeUI(defaultConfig);

// Setup WebSocket handlers
wsService.on('graph_data', (data) => {
  graphManager.updateGraph(data);
});

// Function to request graph data
function requestGraphData() {
  const config = getCurrentSimConfig();
  wsService.requestGraphData(config.nodeCount, 100);
}

// Function to load CSV file
function loadCsvFile() {
  const filePath = uiManager.getFilePath();
  if (!filePath) {
    console.warn('Please enter a file path');
    return;
  }
  wsService.loadCsvFile(filePath);
}

// Function to search nodes
function searchNodes() {
  const searchTerm = uiManager.getSearchTerm();
  if (!searchTerm) {
    graphManager.unselectPoints();
    uiManager.clearNodeDetails();
    return;
  }

  const searchType = uiManager.getSearchType();
  const nodes = graphManager.getCurrentGraphData()?.nodes || [];
  
  const matchingNodes = nodes.filter((node: any) => {
    if (searchType === 'name') {
      return node.name?.toLowerCase().includes(searchTerm);
    } else {
      return node.id?.toLowerCase().includes(searchTerm);
    }
  });

  if (matchingNodes.length === 0) {
    graphManager.unselectPoints();
    uiManager.clearNodeDetails();
    return;
  }

  const allNodesToHighlight = new Set<string>();
  matchingNodes.forEach((node: any) => {
    allNodesToHighlight.add(node.id);
    const childNodes = graphManager.findChildNodes(node.id);
    childNodes.forEach(childId => allNodesToHighlight.add(childId));
  });

  const indices = Array.from(allNodesToHighlight).map(id => 
    nodes.findIndex((n: any) => n.id === id)
  ).filter(i => i !== -1);

  graphManager.selectPointsByIndices(indices);
  graphManager.fitView();
  
  const firstMatchingNodeIndex = nodes.findIndex((node: any) => node.id === matchingNodes[0].id);
  const node = nodes[firstMatchingNodeIndex];
  if (node) {
    const childNodes = graphManager.findChildNodes(node.id);
    uiManager.updateNodeDetails({
      name: node.name || '-',
      id: node.id,
      depth: node.depth?.toString() || '-',
      children: childNodes.length.toString()
    });
  }
}

// Add event listeners for controls
nodeCountInput.addEventListener('change', () => {
  requestGraphData();
});

minDistanceInput.addEventListener('change', () => {
  graphManager.setConfig(getConfig());
});

pointSizeInput.addEventListener('change', () => {
  graphManager.setConfig(getConfig());
});

linkWidthInput.addEventListener('change', () => {
  graphManager.setConfig(getConfig());
});

curvedLinesInput.addEventListener('change', () => {
  graphManager.setConfig(getConfig());
});

spaceSizeInput.addEventListener('change', () => {
  graphManager.setConfig(getConfig());
});

// Add event listeners for simulation controls
disableSimulationInput.addEventListener('change', () => {
  graphManager.setConfig(getConfig());
});

gravityInput.addEventListener('change', () => {
  graphManager.setConfig(getConfig());
});

repulsionInput.addEventListener('change', () => {
  graphManager.setConfig(getConfig());
});

linkSpringInput.addEventListener('change', () => {
  graphManager.setConfig(getConfig());
});

decayInput.addEventListener('change', () => {
  graphManager.setConfig(getConfig());
});

// Simulation Controls
function togglePause() {
  if (uiManager.isGraphPaused()) {
    graphManager.start();
  } else {
    graphManager.pause();
  }
}

function restartSimulation() {
  graphManager.restart();
}

// Add simulation progress update
function updateSimulationProgress() {
  const progress = graphManager.getSimulationProgress();
  if (progress !== undefined) {
    uiManager.updateSimulationProgress(progress);
  }
  if (graphManager.isSimulationRunning()) {
    requestAnimationFrame(updateSimulationProgress);
  }
}

// Demo Actions
function zoomIn() {
  const pointIndex = graphManager.getRandomPointIndex();
  graphManager.zoomToPointByIndex(pointIndex);
  graphManager.selectPointByIndex(pointIndex);
  graphManager.pause();
}

function selectPoint() {
  const pointIndex = graphManager.getRandomPointIndex();
  graphManager.selectPointByIndex(pointIndex);
  graphManager.fitView();
  graphManager.pause();
}

function selectPointsInArea() {
  const w = div.clientWidth;
  const h = div.clientHeight;
  const left = getRandomInRange([w / 4, w / 2]);
  const right = getRandomInRange([left, (w * 3) / 4]);
  const top = getRandomInRange([h / 4, h / 2]);
  const bottom = getRandomInRange([top, (h * 3) / 4]);
  graphManager.pause();
  graphManager.selectPointsInRange([[left, top], [right, bottom]]);
}

// Add event listeners for buttons
document.getElementById('regenerate')?.addEventListener('click', requestGraphData);
document.getElementById('fit-view')?.addEventListener('click', () => graphManager.fitView());
document.getElementById('zoom')?.addEventListener('click', zoomIn);
document.getElementById('select-point')?.addEventListener('click', selectPoint);
document.getElementById('select-points-in-area')?.addEventListener('click', selectPointsInArea);
document.getElementById('search')?.addEventListener('click', searchNodes);
document.getElementById('load-file')?.addEventListener('click', loadCsvFile);

// Add keyboard event listeners
uiManager.nodeSearchInput?.addEventListener('keypress', (e) => {
  if (e.key === 'Enter') {
    searchNodes();
  }
});

uiManager.filePathInput?.addEventListener('keypress', (e) => {
  if (e.key === 'Enter') {
    loadCsvFile();
  }
});

// Request initial graph data
requestGraphData();

// Start progress update loop
updateSimulationProgress();
