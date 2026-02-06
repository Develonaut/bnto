// Package spreadsheet provides CSV and Excel file operations.
//
// The spreadsheet neta supports:
//   - CSV read/write with headers (critical for Phase 8)
//   - Excel read/write using excelize library
//   - Graceful handling of missing/extra columns
//
// CRITICAL FOR PHASE 8: Used to read product CSV files.
// Outputs array of maps: [{"sku": "PROD-001", "name": "Product A"}, ...]
//
// Example CSV read:
//
//	params := map[string]interface{}{
//	    "operation": "read",
//	    "format": "csv",
//	    "path": "products.csv",
//	}
//	result, err := spreadsheetNeta.Execute(ctx, params)
//	// result["rows"] = []map[string]interface{}{ ... }
//
// Learn more about CSV in Go: https://pkg.go.dev/encoding/csv
package spreadsheet

import (
	"context"
	"fmt"
)

// Spreadsheet implements the spreadsheet neta for CSV/Excel operations.
type Spreadsheet struct{}

// New creates a new spreadsheet neta instance.
func New() *Spreadsheet {
	return &Spreadsheet{}
}

// Execute runs spreadsheet operations (read/write CSV or Excel).
//
// Parameters:
//   - operation: "read" or "write"
//   - format: "csv" or "excel"
//   - path: file path
//   - rows: data for write operations ([]map[string]interface{})
//
// Returns for read:
//   - rows: array of maps with column headers as keys
//
// Returns for write:
//   - written: number of rows written
func (s *Spreadsheet) Execute(ctx context.Context, params map[string]interface{}) (interface{}, error) {
	operation, ok := params["operation"].(string)
	if !ok {
		return nil, fmt.Errorf("operation parameter is required (read or write)")
	}

	format, ok := params["format"].(string)
	if !ok {
		return nil, fmt.Errorf("format parameter is required (csv or excel)")
	}

	path, ok := params["path"].(string)
	if !ok {
		return nil, fmt.Errorf("path parameter is required")
	}

	switch operation {
	case "read":
		return s.read(ctx, format, path)
	case "write":
		rows, ok := params["rows"].([]map[string]interface{})
		if !ok {
			return nil, fmt.Errorf("rows parameter is required for write operation")
		}
		return s.write(ctx, format, path, rows)
	default:
		return nil, fmt.Errorf("invalid operation: %s (must be read or write)", operation)
	}
}

// read reads a spreadsheet file and returns rows as array of maps.
func (s *Spreadsheet) read(ctx context.Context, format, path string) (interface{}, error) {
	switch format {
	case "csv":
		return s.readCSV(ctx, path)
	case "excel":
		return s.readExcel(ctx, path)
	default:
		return nil, fmt.Errorf("invalid format: %s (must be csv or excel)", format)
	}
}

// write writes rows to a spreadsheet file.
func (s *Spreadsheet) write(ctx context.Context, format, path string, rows []map[string]interface{}) (interface{}, error) {
	switch format {
	case "csv":
		return s.writeCSV(ctx, path, rows)
	case "excel":
		return s.writeExcel(ctx, path, rows)
	default:
		return nil, fmt.Errorf("invalid format: %s (must be csv or excel)", format)
	}
}
