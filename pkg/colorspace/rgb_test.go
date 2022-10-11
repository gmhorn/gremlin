package colorspace

import (
	"fmt"
	"testing"

	"github.com/gmhorn/gremlin/pkg/spectrum"
)

func TestSRGB_ConvertXYZ(t *testing.T) {
	xyz := CIE1931.Convert(spectrum.Blackbody(9500))
	t.Log("XYZ:", xyz)
	rgb := SRGB.ConvertXYZ(xyz)
	t.Log("RGB", rgb)
}

func TestSRGB_Convert(t *testing.T) {
	for temp := 1000; temp <= 12000; temp += 500 {
		b := spectrum.Blackbody(temp)
		rgb := SRGB.Convert(b)
		t.Log("Temp", temp, "sRGB", rgb)
	}
}

func TestColors(t *testing.T) {
	s := spectrum.Blue
	srgb := SRGB.Convert(s)
	fmt.Println(srgb)
}
