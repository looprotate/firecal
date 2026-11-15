/** One tile and month of Cube's overlap report (`crates/firecal-harmonize/overlap-report.md`). */
export interface OverlapRow {
  tileRow: number;
  tileCol: number;
  monthIndex: number;
  /** The month was used to fit the calibration; its agreement is not a validation. */
  training: boolean;
  modis: number;
  viirs: number;
  harmonized: number;
}

const HEADER =
  'tile_row,tile_col,year,month,month_index,modis_satellite,viirs_satellite,split,raw_modis_count,raw_viirs_count,harmonized_viirs_count';

/** Reads the report's CSV text; refuses another file or a row that is not numbers where numbers belong. */
export function parseOverlapCsv(text: string): OverlapRow[] {
  const [header, ...lines] = text.split('\n').filter((line) => line.trim() !== '');
  if (header?.trim() !== HEADER) throw new Error('This is not the overlap report of firecal-harmonize.');
  return lines.map((line, at) => {
    const fields = line.split(',');
    const number = (column: number): number => {
      const value = Number(fields[column]);
      if (fields[column] === undefined || fields[column] === '' || !Number.isFinite(value) || value < 0) {
        throw new Error(`The overlap report has a bad number in row ${at + 1}, column ${column + 1}.`);
      }
      return value;
    };
    return {
      tileRow: number(0),
      tileCol: number(1),
      monthIndex: number(4),
      training: fields[7] === 'training',
      modis: number(8),
      viirs: number(9),
      harmonized: number(10),
    };
  });
}
