package tree_sitter_Ignis_test

import (
	"testing"

	tree_sitter "github.com/smacker/go-tree-sitter"
	"github.com/tree-sitter/tree-sitter-Ignis"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_Ignis.Language())
	if language == nil {
		t.Errorf("Error loading Ignis grammar")
	}
}
