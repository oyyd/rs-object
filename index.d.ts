export interface SymbolInfo {
  index: number,
  name: string,
  address: number,
  kind: string,
}

export interface SectionInfo {
  index: number,
  address: number,
  size: number,
  align: number,
  file_range?: [number, number],
  name: string,
}

export class File {
  /**
   * Constructor.
   * @param buf Node.js Buffer of your object.
   */
  constructor(buf: Buffer);

  /**
   * Equivalent of https://docs.rs/object/0.27.1/object/read/trait.Object.html#tymethod.symbols
   */
  symbols(): SymbolInfo[];

  /**
   * Equivalent of https://docs.rs/object/0.27.1/object/read/trait.Object.html#tymethod.dynamic_symbols
   */
  dynamic_symbols(): SymbolInfo[];

  /**
   * Equivalent of https://docs.rs/object/0.27.1/object/read/struct.File.html#method.format
   */
  format(): string;

  /**
   * Equivalent of https://docs.rs/object/0.27.1/object/read/trait.Object.html#tymethod.section_by_index
   * @param index
   */
  section_by_index(index: number): SectionInfo;

  /**
   * Equivalent of https://docs.rs/object/0.27.1/object/read/trait.Object.html#method.section_by_name
   * @param index
   */
   section_by_name(name: string): SectionInfo;
}
