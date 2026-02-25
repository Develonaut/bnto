package spreadsheet

import (
	"context"
	"fmt"

	"github.com/xuri/excelize/v2"
)

// readExcel reads an Excel file with headers.
func (s *Spreadsheet) readExcel(ctx context.Context, path string) (interface{}, error) {
	f, err := excelize.OpenFile(path)
	if err != nil {
		return nil, fmt.Errorf("failed to open Excel file: %w", err)
	}
	defer f.Close()

	records, err := readExcelSheet(f)
	if err != nil {
		return nil, err
	}

	if len(records) == 0 {
		return emptyRows(), nil
	}

	rows := convertRecordsToMaps(records)

	return map[string]interface{}{
		"rows": rows,
	}, nil
}

// writeExcel writes rows to an Excel file.
func (s *Spreadsheet) writeExcel(ctx context.Context, path string, rows []map[string]interface{}) (interface{}, error) {
	if len(rows) == 0 {
		return map[string]interface{}{
			"written": 0,
		}, nil
	}

	f := excelize.NewFile()
	defer f.Close()

	sheetName := "Sheet1"
	headers := extractHeaders(rows[0])

	if err := writeExcelHeaders(f, sheetName, headers); err != nil {
		return nil, err
	}

	if err := writeExcelRows(f, sheetName, rows, headers); err != nil {
		return nil, err
	}

	if err := f.SaveAs(path); err != nil {
		return nil, fmt.Errorf("failed to save Excel file: %w", err)
	}

	return map[string]interface{}{
		"written": len(rows),
	}, nil
}

// readExcelSheet reads records from the first sheet of an Excel file.
func readExcelSheet(f *excelize.File) ([][]string, error) {
	sheets := f.GetSheetList()
	if len(sheets) == 0 {
		return [][]string{}, nil
	}

	sheetName := sheets[0]
	records, err := f.GetRows(sheetName)
	if err != nil {
		return nil, fmt.Errorf("failed to read Excel sheet: %w", err)
	}

	return records, nil
}

// writeExcelHeaders writes headers to an Excel sheet.
func writeExcelHeaders(f *excelize.File, sheetName string, headers []string) error {
	for i, header := range headers {
		cell, _ := excelize.CoordinatesToCellName(i+1, 1)
		if err := f.SetCellValue(sheetName, cell, header); err != nil {
			return fmt.Errorf("failed to set header: %w", err)
		}
	}
	return nil
}

// writeExcelRows writes rows to an Excel sheet.
func writeExcelRows(f *excelize.File, sheetName string, rows []map[string]interface{}, headers []string) error {
	for rowIdx, row := range rows {
		for colIdx, header := range headers {
			cell, _ := excelize.CoordinatesToCellName(colIdx+1, rowIdx+2)
			if val, ok := row[header]; ok {
				if err := f.SetCellValue(sheetName, cell, val); err != nil {
					return fmt.Errorf("failed to set cell value: %w", err)
				}
			}
		}
	}
	return nil
}
