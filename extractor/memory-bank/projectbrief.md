# Project Brief: ARMA3 Extractor

## Overview
ARMA3 Extractor is a Rust library designed to handle the extraction and caching of PBO (Packed Bank of Objects) files from the ARMA 3 game. It provides a robust system for scanning directories, extracting PBO contents to a cache location, and managing cache validity based on file modifications and extension filters.

## Core Requirements

1. **PBO Scanning**: Scan directories for PBO files (both game data and mission PBOs).
2. **Efficient Extraction**: Extract PBO contents to a cache location, optimized for performance.
3. **Intelligent Caching**: Only extract PBOs when necessary (when they're modified or the extraction configuration changes).
4. **Extension Filtering**: Support for extracting only specific file types from PBOs.
5. **Parallel Processing**: Utilize multiple threads for improved extraction performance.
6. **File-Based Storage**: Maintain a file-based database of extracted files and their metadata.
7. **Error Handling**: Robust error handling for failed extractions and other issues.
8. **Self-contained Design**: Operate without external database dependencies.

## Technical Goals

1. Create a high-performance PBO extraction system for ARMA 3 tools.
2. Provide a clean API for managing the extraction and caching of PBO files.
3. Support both game data PBOs (addons, etc.) and mission PBOs.
4. Track extraction metadata to optimize repeated operations.
5. Operate as a standalone component with minimal external dependencies.
6. Enable easy source tracking of extracted files back to their original PBOs.

## Components

1. **Scanner**: Responsible for finding PBO files in directories.
2. **Processor**: Handles the actual extraction of PBO contents.
3. **Manager**: Coordinates the scanning, extraction, and caching processes.
4. **FileDb Manager**: Manages the file-based database operations for tracking cache state.
5. **Models**: Defines the data structures used throughout the system.
6. **Error Handling**: Centralized error types and handling mechanisms.

## Scope

The project focuses specifically on PBO extraction and caching, not on other aspects of ARMA 3 modding or mission editing. It is designed to be a component in a larger ecosystem of ARMA 3 tools, but maintains independence by not relying on external database components. 