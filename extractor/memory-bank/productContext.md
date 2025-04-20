# Product Context

## Purpose

The ARMA3 Extractor library exists to solve performance and caching challenges in Arma 3 modding and development tools. Arma 3 stores game content in PBO (Packed Bank of Objects) files, which are compressed archives containing game assets like scripts, models, textures, and configuration files. Tools that work with this content need efficient ways to extract and process these files.

## Problems Solved

1. **Inefficient Repeated Extraction**: Without a caching system, tools would need to extract the same PBO files repeatedly, wasting time and resources.

2. **Large File Management**: Arma 3 mods and missions can contain hundreds of PBO files, totaling gigabytes of data. Extracting all of this content at once is impractical.

3. **Selective File Access**: Most tools only need specific file types from PBOs (such as scripts or configs), not every file.

4. **Modification Tracking**: When PBOs are updated, tools need to know which cached files are no longer valid.

5. **Source Tracking**: Tools need a way to query and find specific files across many extracted PBOs and trace them back to their source.

6. **Dependency Management**: Avoiding unnecessary dependencies makes tools more portable and easier to maintain.

7. **PBO Format Quirks**: Reliably handling edge cases and specific behaviors of the PBO format and associated tooling (like `config.bin` renaming) to ensure accurate file extraction based on user configuration.

## User Experience Goals

1. **Minimal Wait Times**: Users of tools built with this library should experience minimal delays when accessing Arma 3 content.

2. **Transparency**: The extraction process should provide clear feedback about progress and any issues encountered.

3. **Resource Efficiency**: The system should use disk space efficiently by only extracting necessary files.

4. **Reliability**: The extraction and caching should work consistently across different environments and with mods from various sources.

5. **Simplicity**: The library should be easy to integrate with minimal setup requirements.

## Integration Context

ARMA3 Extractor is designed to work as part of a larger ecosystem of Arma 3 development tools. It:

1. Maintains its own file-based database for tracking extracted files and metadata.

2. Provides a foundation for other tools that need to analyze or manipulate Arma 3 content.

3. Uses the `pbo_tools` crate for the actual PBO extraction operations.

4. Implements a clean API that higher-level applications can use without needing to understand the details of PBO extraction.

5. Offers a simple way to trace extracted files back to their source PBOs.

## Intended Usage Patterns

The library supports several key usage patterns:

1. **Batch Extraction**: Processing all game data PBOs at application startup.

2. **On-Demand Extraction**: Extracting specific mission PBOs when needed.

3. **Targeted File Access**: Finding and accessing specific file types (like scripts or configs) across all extracted content.

4. **Incremental Updates**: Only processing PBOs that have changed since the last extraction.

5. **Source Lookup**: Finding which PBO a particular file came from for reference or additional processing. 