package render

import (
	"fmt"
	"image"
	"image/color"
	"image/png"
	"math/rand"
	"os"

	"github.com/gmhorn/gremlin/pkg/colorspace"
	"github.com/gmhorn/gremlin/pkg/spectrum"
	"github.com/gmhorn/gremlin/pkg/util"
)

type FrameSample struct {
	Pixels []Pixel
	Offset int
}

func Render(width, height, binSize int) []Pixel {
	frame := make([]Pixel, width*height)

	routines := 0
	samples := make(chan FrameSample)

	fmt.Println("Starting goroutines")
	for offset := 0; offset < len(frame); {
		size := util.IntMin(binSize, len(frame)-offset)
		fmt.Println("Offset:", offset, "Size:", size)
		go renderSample(offset, size, samples)
		routines++
		offset += size
	}

	fmt.Println("Awaiting goroutines")
	for i := 0; i < routines; i++ {
		sample := <-samples
		fmt.Println("Filling pixels", sample.Offset, "to", sample.Offset+len(sample.Pixels))
		for j := range sample.Pixels {
			frame[j+sample.Offset].Color[0] += sample.Pixels[j].Color[0]
			frame[j+sample.Offset].Color[1] += sample.Pixels[j].Color[1]
			frame[j+sample.Offset].Color[2] += sample.Pixels[j].Color[2]
			frame[j+sample.Offset].Samples += sample.Pixels[j].Samples
		}
	}

	return frame
}

func renderSample(offset, size int, c chan FrameSample) {
	fmt.Printf("Rendering sample, offset: %d, size: %d\n", offset, size)
	randSpec := spectrum.Peak(rand.Intn(780-380) + 380)
	randColor := colorspace.SRGB.Convert(randSpec)
	randColor = randColor.Scale(rand.Float64())
	// randColor := colorspace.Point{
	// 	rand.Float64(),
	// 	rand.Float64(),
	// 	rand.Float64(),
	// }

	pixels := make([]Pixel, size)
	for i := 0; i < size; i++ {
		// TODO we'd calculate real position using offset, plus parent Width and
		// Height here

		pixels[i].Color = randColor
		pixels[i].Samples = 1
	}

	c <- FrameSample{Pixels: pixels, Offset: offset}
}

func OutputImage(width, height int, pixels []Pixel, name string) error {
	fmt.Println("Creating image")
	img := image.NewRGBA(image.Rect(0, 0, width, height))
	for t := range pixels {
		x := t % width
		y := t / width
		c := &color.RGBA{
			R: uint8(255.999 * (pixels[t].Color[0] / float64(pixels[t].Samples))),
			G: uint8(255.999 * (pixels[t].Color[1] / float64(pixels[t].Samples))),
			B: uint8(255.999 * (pixels[t].Color[2] / float64(pixels[t].Samples))),
			A: 255,
		}
		img.Set(x, y, c)
	}

	file, err := os.Create(name)
	if err != nil {
		return err
	}
	defer file.Close()

	return png.Encode(file, img)
}
