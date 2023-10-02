// an implementation of perlin noise
// this implementation does not include frequency, which exists outside the main function
// it is also HEAVILY commented for a clear explanation of what's going on in the algorithm
pub fn perlin(x: f32, y: f32) -> f32 {
    // find the "floor" of these floating point numbers
    let mut xi = x.floor() as usize;
    let mut yi = y.floor() as usize;

    // wrap values around 255 to access permutation table
    xi = xi % 255;
    yi = yi % 255;

    // pop off floating remainder from x and y
    let xf = x - xi as f32;
    let yf = y - yi as f32;

    // find grid point -> input point vectors
    // bot = bottom, r and l = right and left
    // done to keep variables from being too verbose
    let top_r = Vec2 {x: (xf - 1.0), y: (yf - 1.0)};
    let top_l = Vec2 {x: (xf      ), y: (yf - 1.0)};
    let bot_r = Vec2 {x: (xf - 1.0), y: (yf      )};
    let bot_l = Vec2 {x: (xf      ), y: (yf      )};

    // get grid point values from permutation table (defined below)
    let val_top_r = P[P[xi + 1] + yi + 1];
    let val_top_l = P[P[xi    ] + yi + 1];
    let val_bot_r = P[P[xi + 1] + yi    ];
    let val_bot_l = P[P[xi    ] + yi    ];

    // find dot products between input vectors and appropriate constant vectors
    let dot_top_l = top_l.dot(constant_vec(val_top_l));
    let dot_top_r = top_r.dot(constant_vec(val_top_r));
    let dot_bot_r = bot_r.dot(constant_vec(val_bot_r));
    let dot_bot_l = bot_l.dot(constant_vec(val_bot_l));

    // calculate fade for lerp
    let u = fade(x); let v = fade(y);

    // lerp through all dot products to get noise value at coordinate
    lerp(u, 
        lerp(v, dot_bot_l, dot_top_l), 
        lerp(v, dot_bot_r, dot_top_r))
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
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (t * (b - a))
}

// fade function (also called an ease curve -- smooths out lerp)
fn fade(t: f32) -> f32 {
    ((6.0 * t - 15.0)
         * t + 10.0)
         * t * t * t
}

#[derive(Clone, Copy)]
struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    // method for computing the dot product
    fn dot(&self, other: Vec2) -> f32 {
        (self.x * other.x) + (self.y * other.y)
    }
}

// fractal brownian motion -- use this instead of perlin() for map generation.
// a = amplitude, f = frequency, o = octaves
// might have to play around with these values to find the most natural map layout
pub fn fbm(x: f32, y: f32, a: f32, f: f32, o: usize) -> f32 {
    if o == 0 {
        0.0
    }
    else {
        a * perlin(x * f, y * f) + fbm(x, y, 
            a * 0.5, // multiply amplitude by decimal (ex. 0.5) to decrease it
            f * 2.0, // double frequency to increase frequency
            o - 1)
    }
}

// *** planned functions: ***
// map(height, width) -> use fbm function to generate array of values for map of any size
//                    -> clamp values in a range between 0.0 and 1.0 if necessary 
//                       (result += 1.0, result / 2.0)

// permutation table -- step through this with a random "offset" to generate new noise fields
// or shuffle the table to generate new noise fields
// taken from Ken Perlin's implementation, standard values doubled
const P: [usize; 512] = 
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
205, 93, 222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180,
151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7,
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