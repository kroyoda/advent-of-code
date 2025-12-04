package utils

import "strings"

func SplitLines(s string) []string {
	s = strings.ReplaceAll(s, "\r\n", "\n")
	s = strings.ReplaceAll(s, "\r", "\n")
	lines := strings.Split(s, "\n")

	var result []string
	for _, line := range lines {
		if line == "" {
			continue
		}
		result = append(result, strings.TrimSpace(line))
	}
	return result
}
