package main

import (
	"image/png"
	"log"
	"os"

	"github.com/gmhorn/gremlin/pkg/colorspace"
	"github.com/gmhorn/gremlin/pkg/render"
	"github.com/gmhorn/gremlin/pkg/spectrum"
)

func main() {
	if err := doBlackbody("blackbody.png"); err != nil {
		log.Fatal(err)
	}
}

func doBlackbody(path string) error {
	temps := make([]float64, 0)
	for temp := 2000; temp <= 12000; temp += 1000 {
		temps = append(temps, float64(temp))
	}

	f := render.NewFilm(100, 100*len(temps))
	for i, temp := range temps {
		b := spectrum.Blackbody(temp)
		for x := 0; x < f.Width; x++ {
			for y := 100 * i; y < 100*(i+1); y++ {
				f.Add(x, y, b)
			}
		}
	}

	file, err := os.Create(path)
	if err != nil {
		return err
	}
	defer file.Close()

	return png.Encode(file, f.Image(colorspace.SRGB))
}
