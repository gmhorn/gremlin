package render

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestRTOW(t *testing.T) {
	err := RTOW()
	assert.NoError(t, err)
}
