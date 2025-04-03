import { NodeDetails } from './types';

export class UIManager {
  private nodeNameElement: HTMLSpanElement;
  private nodeIdElement: HTMLSpanElement;
  private nodeDepthElement: HTMLSpanElement;
  private nodeChildrenElement: HTMLSpanElement;
  private nodeParentElement: HTMLSpanElement;
  private nodeContainerElement: HTMLSpanElement;
  private nodeSourceElement: HTMLSpanElement;
  private spaceSizeInput: HTMLInputElement;
  private isPaused: boolean = false;

  constructor(
  ) {
    this.nodeNameElement = document.getElementById('node-name') as HTMLSpanElement;
    this.nodeIdElement = document.getElementById('node-id') as HTMLSpanElement;
    this.nodeDepthElement = document.getElementById('node-depth') as HTMLSpanElement;
    this.nodeChildrenElement = document.getElementById('node-children') as HTMLSpanElement;
    this.nodeParentElement = document.getElementById('node-parent') as HTMLSpanElement;
    this.nodeContainerElement = document.getElementById('node-container') as HTMLSpanElement;
    this.nodeSourceElement = document.getElementById('node-source') as HTMLSpanElement;
    this.spaceSizeInput = document.getElementById('space-size') as HTMLInputElement;
  }

  public updateNodeDetails(details: NodeDetails) {
    this.nodeNameElement.textContent = details.name;
    this.nodeIdElement.textContent = details.id;
    this.nodeDepthElement.textContent = details.depth;
    this.nodeChildrenElement.textContent = details.children;
    this.nodeParentElement.textContent = details.parent;
    this.nodeContainerElement.textContent = details.container;
    this.nodeSourceElement.textContent = details.source;
  }

  public getSpaceSize(): number {
    return parseInt(this.spaceSizeInput.value) || 4096;
  }

  public isGraphPaused(): boolean {
    return this.isPaused;
  }

  public clearNodeDetails() {
    this.updateNodeDetails({
      name: '-',
      id: '-',
      depth: '-',
      children: '-',
      parent: '-',
      container: '-',
      source: '-'
    });
  }

  public setSpaceSize(value: number) {
    this.spaceSizeInput.value = value.toString();
  }
} 