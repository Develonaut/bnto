package spreadsheet

import (
	"context"
	"encoding/csv"
	"fmt"
	"os"
)

// readCSV reads a CSV file with headers.
func (s *Spreadsheet) readCSV(ctx context.Context, path string) (interface{}, error) {
	file, err := os.Open(path)
	if err != nil {
		return nil, fmt.Errorf("failed to open CSV file: %w", err)
	}
	defer file.Close()

	records, err := readAllRecords(file)
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

// writeCSV writes rows to a CSV file.
func (s *Spreadsheet) writeCSV(ctx context.Context, path string, rows []map[string]interface{}) (interface{}, error) {
	if len(rows) == 0 {
		return map[string]interface{}{
			"written": 0,
		}, nil
	}

	file, err := os.Create(path)
	if err != nil {
		return nil, fmt.Errorf("failed to create CSV file: %w", err)
	}
	defer file.Close()

	writer := csv.NewWriter(file)
	defer writer.Flush()

	headers := extractHeaders(rows[0])

	if err := writer.Write(headers); err != nil {
		return nil, fmt.Errorf("failed to write headers: %w", err)
	}

	for _, row := range rows {
		record := buildRecord(row, headers)
		if err := writer.Write(record); err != nil {
			return nil, fmt.Errorf("failed to write row: %w", err)
		}
	}

	return map[string]interface{}{
		"written": len(rows),
	}, nil
}

// readAllRecords reads all CSV records from a file.
func readAllRecords(file *os.File) ([][]string, error) {
	reader := csv.NewReader(file)
	reader.FieldsPerRecord = -1 // Allow variable number of fields per record
	records, err := reader.ReadAll()
	if err != nil {
		return nil, fmt.Errorf("failed to read CSV: %w", err)
	}
	return records, nil
}

// convertRecordsToMaps converts CSV records to array of maps.
func convertRecordsToMaps(records [][]string) []map[string]interface{} {
	if len(records) == 0 {
		return []map[string]interface{}{}
	}

	headers := records[0]
	rows := make([]map[string]interface{}, 0, len(records)-1)

	for i := 1; i < len(records); i++ {
		row := mapRecordToHeaders(records[i], headers)
		rows = append(rows, row)
	}

	return rows
}

// mapRecordToHeaders maps a record to headers (handling missing/extra columns).
func mapRecordToHeaders(record []string, headers []string) map[string]interface{} {
	row := make(map[string]interface{})

	for j, header := range headers {
		if j < len(record) {
			row[header] = record[j]
		} else {
			row[header] = "" // Missing column
		}
	}

	return row
}

// extractHeaders extracts headers from the first row.
func extractHeaders(firstRow map[string]interface{}) []string {
	headers := make([]string, 0)
	for key := range firstRow {
		headers = append(headers, key)
	}
	return headers
}

// buildRecord builds a CSV record from a row map.
func buildRecord(row map[string]interface{}, headers []string) []string {
	record := make([]string, len(headers))
	for i, header := range headers {
		if val, ok := row[header]; ok {
			record[i] = fmt.Sprintf("%v", val)
		}
	}
	return record
}

// emptyRows returns an empty rows response.
func emptyRows() map[string]interface{} {
	return map[string]interface{}{
		"rows": []map[string]interface{}{},
	}
}
