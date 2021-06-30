
func MandelbrotConcurrent(n int, tb testing.TB) {
	img := image.NewRGBA(image.Rect(0, 0, width, height))
	var wg sync.WaitGroup
	wg.Add(n)
	for i := 0; i < n; i++ {
		go func(id int) {
			t1 := time.Now()
			defer wg.Done()
			defer func() {
				log.Printf("worker %d of %d done in %s", id, n, time.Now().Sub(t1))
			}()
			for py := 0; py < height; py++ {
				y := float64(py)/height*(ymax-ymin) + ymin
				for px := id; px < width; px += n { // iからnおき. n=2なら0,2,4,...と1,3,5,...
					x := float64(px)/width*(xmax-xmin) + xmin
					z := complex(x, y)
					c := mandelbrot(z)
					img.Set(px, py, c) // スライスに書き込むindexがすべて異なるためthread safe
				}
			}
		}(i)
	}
	wg.Wait()
	writeTestImage(tb, img)
}