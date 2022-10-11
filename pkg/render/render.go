package render

import (
	"fmt"
	"math/rand"

	"github.com/gmhorn/gremlin/pkg/camera"
	"github.com/gmhorn/gremlin/pkg/colorspace"
	"github.com/gmhorn/gremlin/pkg/shape"
	"github.com/gmhorn/gremlin/pkg/spectrum"
	"github.com/gmhorn/gremlin/pkg/util"
)

const tileSize = 64

func Fixed(film *camera.Film, cam *camera.Perspective, scene []shape.Shape) error {
	// Split up film into tiles
	tiles := util.Partition(len(film.Pixels), tileSize)
	results := make(chan *camera.FilmTile)

	for _, tile := range tiles {
		go renderTile(tile.Offset, tile.Size, results)
	}

	for range tiles {
		film.Merge(<-results)
	}

	return nil
}

func renderTile(offset, size int, ch chan *camera.FilmTile) {
	fmt.Printf("Rendering tile, offset: %d, size: %d\n", offset, size)

	// Just pick a random color
	randSpec := spectrum.Peak((780-380)*rand.Float64()+380.0, 1.0)
	randColor := colorspace.CIE1931.Convert(randSpec)
	randColor = randColor.Scale(rand.Float64())

	pixels := make([]camera.Pixel, size)
	for i := 0; i < size; i++ {
		// TODO we'd use the real position using the offset

		pixels[i].Color = randColor
		pixels[i].Samples++
	}

	ch <- &camera.FilmTile{Offset: offset, Pixels: pixels}
}
