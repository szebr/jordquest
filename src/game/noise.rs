use rand_chacha::ChaChaRng;
use rand_chacha::rand_core::SeedableRng;
use rand::Rng;

// an implementation of perlin noise
// this implementation does not include frequency, which exists outside the main function
// it is also HEAVILY commented for a clear explanation of what's going on in the algorithm
pub struct Perlin {
    p: [usize; 256],
    amp: f64,
    freq: f64,
    oct: usize,
}

impl Perlin {
    pub fn new(seed: u64, amp: f64, freq: f64, oct: usize) -> Self {
        let p = Self::shuffle(seed);
        Self { p, amp, freq, oct }
    }

    pub fn noise(&self, x: usize, y: usize) -> f64 {
        self.fbm(x as f64, y as f64, self.amp, self.freq, self.oct)
    }

    fn fbm(&self, x: f64, y: f64, a: f64, f: f64, o: usize) -> f64 {
        if o == 0 {
            0.0
        }
        else {
            let r = a * (self.perlin(x * f, y * f) + self.fbm(x, y,
                a * 0.3, // multiply amplitude by decimal (ex. 0.5) to decrease it
                f * 2.0, // double frequency to increase frequency
                o - 1));
            // println!("r: {}", r);
            r
        }
    }

    fn perlin(&self, x: f64, y: f64) -> f64 {
        // find the "floor" of these floating point numbers
        let mut xi = x.floor() as usize;
        let mut yi = y.floor() as usize;
    
        // wrap values around 255 to access permutation table
        xi = xi & 255;
        yi = yi & 255;
    
        // pop off floating remainder from x and y
        let xf = x - xi as f64;
        let yf = y - yi as f64;
    
        // find grid point -> input point vectors
        // bot = bottom, r and l = right and left
        // done to keep variables from being too verbose
        let top_r = Vec2 {x: (xf - 1.0), y: (yf - 1.0)};
        let top_l = Vec2 {x: (xf      ), y: (yf - 1.0)};
        let bot_r = Vec2 {x: (xf - 1.0), y: (yf      )};
        let bot_l = Vec2 {x: (xf      ), y: (yf      )};
        
        // get grid point values from permutation table
        let val_top_r = self.p[(self.p[(xi + 1) & 255] + yi + 1) & 255];
        let val_top_l = self.p[(self.p[(xi    ) & 255] + yi + 1) & 255];
        let val_bot_r = self.p[(self.p[(xi + 1) & 255] + yi    ) & 255];
        let val_bot_l = self.p[(self.p[(xi    ) & 255] + yi    ) & 255];
    
        // find dot products between input vectors and appropriate constant vectors
        let dot_top_l = top_l.dot(Self::constant_vec(val_top_l));
        let dot_top_r = top_r.dot(Self::constant_vec(val_top_r));
        let dot_bot_r = bot_r.dot(Self::constant_vec(val_bot_r));
        let dot_bot_l = bot_l.dot(Self::constant_vec(val_bot_l));
    
        // calculate fade for lerp
        let u = Self::fade(xf); let v = Self::fade(yf);
    
        // lerp through all dot products to get noise value at coordinate
        let n = Self::lerp(Self::lerp(dot_bot_l, dot_top_l, v),
             Self::lerp(dot_bot_r, dot_top_r, v),
             u);
        
        (n + 1.0 as f64) / 2 as f64
    }

    // find the constant vector to use in calculating the dot product of the constant vectors vs input vectors
    fn constant_vec(v: usize) -> Vec2 {
        let h = v % 3;
        let mut vec = Vec2 {x: 1.0, y: -1.0};

        if h == 0 {
            vec.y = 1.0;
        }
        if h == 1 {
            vec.x = -1.0; vec.y = 1.0;
        }
        if h == 2 {
            vec.x = -1.0;
        }

        vec
    }

    // linearly interpolate between a and b, "lerp"
    fn lerp(a: f64, b: f64, t: f64) -> f64 {
        a + t * (b - a)
    }

    // fade function (also called an ease curve -- smooths out lerp)
    fn fade(t: f64) -> f64 {
        ((6.0 * t - 15.0)
            * t + 10.0)
            * t * t * t
    }

    // *** planned functions: ***
    // map(height, width) -> use fbm function to generate array of values for map of any size
    //                    -> clamp values in a range between 0.0 and 1.0 if necessary 
    //                       (result += 1.0, result / 2.0)

    // permutation table -- shuffle the table to generate new noise fields
    // taken from Ken Perlin's implementation
    const P: [usize; 256] = 
    [151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7,
    225, 140, 36, 103, 30, 69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247,
    120, 234, 75, 0, 26, 197, 62, 94, 252, 219, 203, 117, 35, 11, 32, 57, 177, 33,
    88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175, 74, 165, 71, 134,
    139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230, 220,
    105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80,
    73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86,
    164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38,
    147, 118, 126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189,
    28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153, 101,
    155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12,
    191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181,
    199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236,
    205, 93, 222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180];

    // function to shuffle permutation table and return new array of shuffled values
    fn shuffle(seed: u64) -> [usize; 256] {
        // use rand_chacha crate to generate random numbers
        let mut rng = ChaChaRng::seed_from_u64(seed);
        // must create a new array because P is a const
        let mut new_array = Self::P;
        // swap values in permutation table 256 times
        for i in 0..256 {
            let j = rng.gen_range(0..256);
            let temp = Self::P[i];
            new_array[i] = Self::P[j];
            new_array[j] = temp;
        }
        // return the new array
        new_array
    }
}

#[derive(Clone, Copy)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    // method for computing the dot product
    fn dot(&self, other: Vec2) -> f64 {
        (self.x * other.x) + (self.y * other.y)
    }
}
