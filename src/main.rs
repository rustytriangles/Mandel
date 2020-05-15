extern crate image;
use num_complex::Complex64;

fn generate(ll: Complex64, ur: Complex64, width: usize) -> Vec<Vec<Complex64>> {
    let step = (ur.re - ll.re) / (width as f64);
    let height = ((ur.im - ll.im) / step) as usize;

    let mut ret = Vec::with_capacity(height);
    for y in 0..height {
	let mut scanline = Vec::with_capacity(width);
	for x in 0..width {
	    scanline.push(Complex64::new(ll.re + (x as f64) * step,
					 ll.im + (y as f64) * step));
	}
	ret.push(scanline);
    }
    ret
}

fn create_empty(width: usize, height: usize) -> Vec<Vec<Complex64>> {
    let mut dst = Vec::with_capacity(height);
    for _j in 0..height {
	let mut scanline = Vec::with_capacity(width);
	for _i in 0..width {
	    scanline.push(Complex64::new(0.,0.));
	}
	dst.push(scanline);
    }
    dst
}

fn create_counter(width: usize, height: usize) -> Vec<Vec<usize>> {
    let mut dst = Vec::with_capacity(height);
    for _j in 0..height {
	let mut scanline = Vec::with_capacity(width);
	for _i in 0..width {
	    scanline.push(0 as usize);
	}
	dst.push(scanline);
    }
    dst
}

fn choose_color(p: Complex64, _n: usize, _max: usize) -> image::Rgb<u8> {

    static WHITE: image::Rgb<u8> = image::Rgb([255,255,255]);
    static BLACK: image::Rgb<u8> = image::Rgb([0,0,0]);
    if p.re > 0. {
	if p.im > 0. {
	    WHITE
	} else {
	    BLACK
	}
    } else {
	if p.im > 0. {
	    BLACK
	} else {
	    WHITE
	}
    }
}

fn main() {

    let width : usize = 1280 * 1;
    //    let c = generate(Complex64::new(-2.,-1.), Complex::new(1.,1.), width);
    let c = Complex64::new(-0.743643887035763,0.13182590421259918);
    let hw = 1.5e-4;
    let hh = hw / 1.78;
    let c = generate(c - Complex64::new(hw, hh), c + Complex64::new(hw, hh), width);
    let height = c.len();

    let mut z: Vec<Vec<Complex64>> = create_empty(width, height);

    let mut imgbuf = image::ImageBuffer::new(width as u32, height as u32);

    let first_frame = 58;
    let max_count = 250;
    let mut ctr: Vec<Vec<usize>> = create_counter(width, height);
    for n in 0..max_count {
	for y in 0..height {
	    for x in 0..width {
		if ctr[y][x] == 0 {
		    let newval = z[y][x] * z[y][x] + c[y][x];
		    if newval.norm_sqr() > 4. {
			ctr[y][x] = n;
		    }
		    z[y][x] =  newval;
		}
	    }
	}

	if n >= first_frame {
	    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
		*pixel = choose_color(z[y as usize][x as usize],ctr[y as usize][x as usize], max_count);
	    }
	    let filename = format!("fractal{:04}.png",n);
	    imgbuf.save(filename).unwrap();
	}
    }

}
