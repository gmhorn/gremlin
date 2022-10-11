package spectrum

import "testing"

var benchResultSampled *Sampled

func BenchmarkSampled_Scale(b *testing.B) {
	dist := Sample(Blackbody(4500))
	for i := 0; i < b.N; i++ {
		benchResultSampled = dist.Scale(float64(i))
	}
}

func BenchmarkSampled_Plus(b *testing.B) {
	x := Sample(Blackbody(4500))
	y := Sample(Flat(0.1))

	for i := 0; i < b.N; i++ {
		benchResultSampled = x.Plus(y)
	}
}
