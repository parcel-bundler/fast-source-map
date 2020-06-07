// @flow
import type { ParsedMap, VLQMap, SourceMapStringifyOptions, IndexedMapping } from './types';

import path from 'path';
import { generateInlineMap, partialVlqMapToSourceMap } from './utils';

export default class SourceMap {
  sourceMapInstance: any;

  /**
   * Generates an empty map from the provided fileName and sourceContent
   *
   * @param sourceName path of the source file
   * @param sourceContent content of the source file
   * @param lineOffset an offset that gets added to the sourceLine index of each mapping
   */
  static generateEmptyMap(sourceName: string, sourceContent: string, lineOffset: number = 0): SourceMap {
    throw new Error('SourceMap.generateEmptyMap() must be implemented when extending SourceMap');
  }

  /**
   * Generates an empty map from the provided fileName and sourceContent
   *
   * @param sourceName path of the source file
   * @param sourceContent content of the source file
   * @param lineOffset an offset that gets added to the sourceLine index of each mapping
   */
  addEmptyMap(sourceName: string, sourceContent: string, lineOffset: number = 0): SourceMap {
    this.sourceMapInstance.addEmptyMap(sourceName, sourceContent, lineOffset);
    return this;
  }

  /**
   * Appends raw VLQ mappings to the sourcemaps
   */
  addRawMappings(map: VLQMap, lineOffset: number = 0, columnOffset: number = 0): SourceMap {
    let { sourcesContent, sources = [], mappings, names = [] } = map;
    if (!sourcesContent) {
      sourcesContent = sources.map(() => '');
    } else {
      sourcesContent = sourcesContent.map((content) => (content ? content : ''));
    }
    this.sourceMapInstance.addRawMappings(
      mappings,
      sources,
      sourcesContent.map((content) => (content ? content : '')),
      names,
      lineOffset,
      columnOffset
    );
    return this;
  }

  /**
   * Appends a flatbuffer to this sourcemap
   * Note: The flatbuffer buffer should be generated by this library
   *
   * @param buffer the sourcemap buffer that should get appended to this sourcemap
   * @param lineOffset an offset that gets added to the sourceLine index of each mapping
   * @param columnOffset  an offset that gets added to the sourceColumn index of each mapping
   */
  addBufferMappings(buffer: Buffer, lineOffset: number = 0, columnOffset: number = 0): SourceMap {
    this.sourceMapInstance.addBufferMappings(buffer, lineOffset, columnOffset);
    return this;
  }

  /**
   * Appends a Mapping object to this sourcemap
   * Note: line numbers start at 1 due to mozilla's source-map library
   *
   * @param mapping the mapping that should be appended to this sourcemap
   * @param lineOffset an offset that gets added to the sourceLine index of each mapping
   * @param columnOffset  an offset that gets added to the sourceColumn index of each mapping
   */
  addIndexedMapping(mapping: IndexedMapping<string>, lineOffset?: number = 0, columnOffset?: number = 0): void {
    let hasValidOriginal =
      mapping.original &&
      typeof mapping.original.line === 'number' &&
      !isNaN(mapping.original.line) &&
      typeof mapping.original.column === 'number' &&
      !isNaN(mapping.original.column);

    this.sourceMapInstance.addIndexedMapping(
      mapping.generated.line + lineOffset - 1,
      mapping.generated.column + columnOffset,
      // $FlowFixMe
      hasValidOriginal ? mapping.original.line - 1 : -1,
      // $FlowFixMe
      hasValidOriginal ? mapping.original.column : -1,
      mapping.source || '',
      mapping.name || ''
    );
  }

  /**
   * Appends an array of Mapping objects to this sourcemap
   * This is useful when improving performance if a library provides the non-serialised mappings
   *
   * Note: This is only faster if they generate the serialised map lazily
   * Note: line numbers start at 1 due to mozilla's source-map library
   *
   * @param mappings an array of mapping objects
   * @param lineOffset an offset that gets added to the sourceLine index of each mapping
   * @param columnOffset  an offset that gets added to the sourceColumn index of each mapping
   */
  addIndexedMappings(
    mappings: Array<IndexedMapping<string>>,
    lineOffset?: number = 0,
    columnOffset?: number = 0
  ): SourceMap {
    for (let mapping of mappings) {
      this.addIndexedMapping(mapping, lineOffset, columnOffset);
    }
    return this;
  }

  /**
   * Appends a name to the sourcemap
   *
   * @param name the name that should be appended to the names array
   * @returns the index of the added name in the names array
   */
  addName(name: string): number {
    return this.sourceMapInstance.addName(name);
  }

  /**
   * Appends an array of names to the sourcemap's names array
   *
   * @param names an array of names to add to the sourcemap
   * @returns an array of indexes of the names in the sourcemap's names array, has the same order as the provided names array
   */
  addNames(names: Array<string>): Array<number> {
    return names.map((n) => this.addName(n));
  }

  /**
   * Appends a source to the sourcemap's sources array
   *
   * @param source a filepath that should be appended to the sources array
   * @returns the index of the added source filepath in the sources array
   */
  addSource(source: string): number {
    return this.sourceMapInstance.addSource(source);
  }

  /**
   * Appends an array of sources to the sourcemap's sources array
   *
   * @param sources an array of filepaths which should sbe appended to the sources array
   * @returns an array of indexes of the sources that have been added to the sourcemap, returned in the same order as provided in the argument
   */
  addSources(sources: Array<string>): Array<number> {
    return sources.map((s) => this.addSource(s));
  }

  /**
   * Get the index in the sources array for a certain source file filepath
   *
   * @param source the filepath of the source file
   */
  getSourceIndex(source: string): number {
    return this.sourceMapInstance.getSourceIndex(source);
  }

  /**
   * Get the source file filepath for a certain index of the sources array
   *
   * @param index the index of the source in the sources array
   */
  getSource(index: number): string {
    return this.sourceMapInstance.getSource(index);
  }

  setSourceContent(sourceName: string, sourceContent: string): void {
    return this.sourceMapInstance.setSourceContent(sourceName, sourceContent);
  }

  getSourceContent(sourceName: string): string {
    return this.sourceMapInstance.getSourceContent(sourceName);
  }

  /**
   * Get the index in the names array for a certain name
   *
   * @param name the name you want to find the index of
   */
  getNameIndex(name: string): number {
    return this.sourceMapInstance.getNameIndex(name);
  }

  /**
   * Get the name for a certain index of the names array
   *
   * @param index the index of the name in the names array
   */
  getName(index: number): string {
    return this.sourceMapInstance.getName(index);
  }

  /**
   * Convert a Mapping object that uses indexes for name and source to the actual value of name and source
   *
   * Note: This is only used internally, should not be used externally and will probably eventually get
   * handled directly in C++ for improved performance
   *
   * @param index the Mapping that should get converted to a string-based Mapping
   */
  indexedMappingToStringMapping(mapping: ?IndexedMapping<number>): ?IndexedMapping<string> {
    if (!mapping) return mapping;

    if (mapping.source != null && mapping.source > -1) {
      // $FlowFixMe
      mapping.source = this.getSource(mapping.source);
    }

    if (mapping.name != null && mapping.name > -1) {
      // $FlowFixMe
      mapping.name = this.getName(mapping.name);
    }

    // $FlowFixMe
    return mapping;
  }

  /**
   * Remaps original positions from this map to the ones in the provided map
   *
   * This works by finding the closest generated mapping in the provided map
   * to original mappings of this map and remapping those to be the original
   * mapping of the provided map.
   *
   * @param buffer exported SourceMap as a flatbuffer
   */
  extends(buffer: Buffer): SourceMap {
    this.sourceMapInstance.extends(buffer);
    return this;
  }

  /**
   * Returns an object with mappings, sources and names
   * This should only be used for tests, debugging and visualising sourcemaps
   *
   * Note: This is a fairly slow operation
   */
  getMap(): ParsedMap {
    return this.sourceMapInstance.getMap();
  }

  /**
   * Searches through the sourcemap and returns a mapping that is close to the provided generated line and column
   *
   * @param line the line in the generated code (starts at 1)
   * @param column the column in the generated code (starts at 0)
   */
  findClosestMapping(line: number, column: number): ?IndexedMapping<string> {
    throw new Error('SourceMap.findClosestMapping() must be implemented when extending SourceMap');
  }

  /**
   * Returns a flatbuffer that represents this sourcemap, used for caching
   */
  toBuffer(): Buffer {
    throw new Error('SourceMap.toBuffer() must be implemented when extending SourceMap');
  }

  /**
   * Returns a serialised map using VLQ Mappings
   */
  toVLQ(): VLQMap {
    return this.sourceMapInstance.stringify();
  }

  /**
   * A function that has to be called at the end of the SourceMap's lifecycle to ensure all memory and native bindings get de-allocated
   */
  delete() {
    throw new Error('SourceMap.delete() must be implemented when extending SourceMap');
  }

  /**
   * Returns a serialised map
   *
   * @param options options used for formatting the serialised map
   */
  async stringify(options: SourceMapStringifyOptions): Promise<string | VLQMap> {
    return partialVlqMapToSourceMap(this.toVLQ(), options);
  }
}
