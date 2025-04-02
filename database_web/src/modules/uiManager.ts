import { NodeDetails } from './types';
import { PlaybackControls } from './playbackControls';

export class UIManager {
  private nodeNameElement: HTMLSpanElement;
  private nodeIdElement: HTMLSpanElement;
  private nodeDepthElement: HTMLSpanElement;
  private nodeChildrenElement: HTMLSpanElement;
  public nodeSearchInput: HTMLInputElement;
  public filePathInput: HTMLInputElement;
  private searchTypeRadios: NodeListOf<HTMLInputElement>;
  private spaceSizeInput: HTMLInputElement;
  private isPaused: boolean = false;
  private playbackControls: PlaybackControls;

  constructor(
    onPlayPause: () => void,
    onRestart: () => void
  ) {
    this.nodeNameElement = document.getElementById('node-name') as HTMLSpanElement;
    this.nodeIdElement = document.getElementById('node-id') as HTMLSpanElement;
    this.nodeDepthElement = document.getElementById('node-depth') as HTMLSpanElement;
    this.nodeChildrenElement = document.getElementById('node-children') as HTMLSpanElement;
    this.nodeSearchInput = document.getElementById('node-search') as HTMLInputElement;
    this.filePathInput = document.getElementById('file-path') as HTMLInputElement;
    this.searchTypeRadios = document.getElementsByName('search-type') as NodeListOf<HTMLInputElement>;
    this.spaceSizeInput = document.getElementById('space-size') as HTMLInputElement;

    // Initialize playback controls
    const actionsContainer = document.querySelector('.actions') as HTMLDivElement;
    this.playbackControls = new PlaybackControls(actionsContainer, this, onPlayPause, onRestart);
  }

  public updateNodeDetails(details: NodeDetails) {
    this.nodeNameElement.textContent = details.name;
    this.nodeIdElement.textContent = details.id;
    this.nodeDepthElement.textContent = details.depth;
    this.nodeChildrenElement.textContent = details.children;
  }

  public getSearchTerm(): string {
    return this.nodeSearchInput.value.toLowerCase();
  }

  public getSearchType(): string {
    return Array.from(this.searchTypeRadios).find(radio => radio.checked)?.value || 'name';
  }

  public getFilePath(): string {
    return this.filePathInput.value.trim();
  }

  public getSpaceSize(): number {
    return parseInt(this.spaceSizeInput.value) || 4096;
  }

  public updateSimulationProgress(progress: number) {
    this.simulationProgress = progress;
    this.playbackControls.updateProgress(progress);
  }

  public setPaused(paused: boolean) {
    this.isPaused = paused;
    this.playbackControls.setPaused(paused);
  }

  public isGraphPaused(): boolean {
    return this.isPaused;
  }

  public clearNodeDetails() {
    this.updateNodeDetails({
      name: '-',
      id: '-',
      depth: '-',
      children: '-'
    });
  }

  public setSpaceSize(value: number) {
    this.spaceSizeInput.value = value.toString();
  }
} 