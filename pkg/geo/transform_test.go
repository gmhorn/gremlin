package geo

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestShift(t *testing.T) {
	m := Shift(Vec{10, 20, 30})
	assert.Equal(t, 10, m[0][3])
}

func TestLookAt(t *testing.T) {
	from := Vec{1, 1, 1}
	to := Origin
	actual := LookAt(from, to, YAxis)

	t.Logf("%0.5f %0.5f %0.5f %0.5f", actual[0][0], actual[0][1], actual[0][2], actual[0][3])
	t.Logf("%0.5f %0.5f %0.5f %0.5f", actual[1][0], actual[1][1], actual[1][2], actual[1][3])
	t.Logf("%0.5f %0.5f %0.5f %0.5f", actual[2][0], actual[2][1], actual[2][2], actual[2][3])
	t.Logf("%0.5f %0.5f %0.5f %0.5f", actual[3][0], actual[3][1], actual[3][2], actual[3][3])
}
