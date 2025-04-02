import { UIManager } from './uiManager';

export class PlaybackControls {
  private container: HTMLDivElement;
  private playPauseButton: HTMLDivElement | null = null;
  private restartButton: HTMLDivElement | null = null;
  private progressBar: HTMLDivElement | null = null;
  private isPaused: boolean = false;
  private onPlayPause: () => void;
  private onRestart: () => void;

  constructor(
    container: HTMLDivElement, 
    uiManager: UIManager,
    onPlayPause: () => void,
    onRestart: () => void
  ) {
    this.container = container;
    this.onPlayPause = onPlayPause;
    this.onRestart = onRestart;
    this.createControls(uiManager);
  }

  private createControls(uiManager: UIManager) {
    // Create container for playback controls
    const controlsContainer = document.createElement('div');
    controlsContainer.className = 'playback-controls';
    this.container.appendChild(controlsContainer);

    // Create play/pause button
    this.playPauseButton = document.createElement('div');
    this.playPauseButton.className = 'playback-button play-pause';
    this.playPauseButton.innerHTML = this.getPlayIcon();
    this.playPauseButton.addEventListener('click', () => {
      if (this.isPaused) {
        this.playPauseButton!.innerHTML = this.getPauseIcon();
        uiManager.setPaused(false);
      } else {
        this.playPauseButton!.innerHTML = this.getPlayIcon();
        uiManager.setPaused(true);
      }
      this.isPaused = !this.isPaused;
      this.onPlayPause();
    });
    controlsContainer.appendChild(this.playPauseButton);

    // Create restart button
    this.restartButton = document.createElement('div');
    this.restartButton.className = 'playback-button restart';
    this.restartButton.innerHTML = this.getRestartIcon();
    this.restartButton.addEventListener('click', () => {
      uiManager.setPaused(false);
      this.playPauseButton!.innerHTML = this.getPauseIcon();
      this.isPaused = false;
      this.onRestart();
    });
    controlsContainer.appendChild(this.restartButton);

    // Create progress bar
    this.progressBar = document.createElement('div');
    this.progressBar.className = 'simulation-progress';
    this.progressBar.innerHTML = '<div class="progress-fill"></div>';
    controlsContainer.appendChild(this.progressBar);
  }

  public updateProgress(progress: number) {
    if (this.progressBar) {
      const fill = this.progressBar.querySelector('.progress-fill') as HTMLDivElement;
      fill.style.width = `${progress * 100}%`;
    }
  }

  public setPaused(paused: boolean) {
    this.isPaused = paused;
    if (this.playPauseButton) {
      this.playPauseButton.innerHTML = paused ? this.getPlayIcon() : this.getPauseIcon();
    }
  }

  private getPlayIcon(): string {
    return `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
      <path d="M8 5v14l11-7z"/>
    </svg>`;
  }

  private getPauseIcon(): string {
    return `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
      <path d="M6 4h4v16H6V4zm8 0h4v16h-4V4z"/>
    </svg>`;
  }

  private getRestartIcon(): string {
    return `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
      <path d="M12 5V1L7 6l5 5V7c3.31 0 6 2.69 6 6s-2.69 6-6 6-6-2.69-6-6H4c0 4.42 3.58 8 8 8s8-3.58 8-8-3.58-8-8-8z"/>
    </svg>`;
  }
} 