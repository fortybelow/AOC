
#[derive(Debug)]
struct Layer {
    pixels: Vec<u8>,
    id: usize,
    shape: (usize, usize),
}

impl Layer {
    fn new(pixels: Vec<u8>, id: usize, shape: (usize, usize)) -> Self {
        Self { pixels, id, shape }
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self.pixels[y * self.shape.0 + x]
    }

    fn set(&mut self, x: usize, y: usize, value: u8) {
        self.pixels[y * self.shape.0 + x] = value;
    }

    fn get_id(&self) -> usize {
        self.id
    }

    fn get_width(&self) -> usize {
        self.shape.0
    }

    fn get_height(&self) -> usize {
        self.shape.1
    }

    fn count(&self, value: u8) -> usize {
        self.pixels.iter().filter(|&&x| x == value).count()
    }
}

#[derive(Debug)]
struct Image {
    layers: Vec<Layer>,
    shape: (usize, usize),
}

impl Image {
    fn new(input: &str, shape: (usize, usize)) -> Self {
        let pixels = input
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect::<Vec<_>>();
        let layer_size = shape.0 * shape.1;
        let layers = pixels
            .chunks_exact(layer_size)
            .enumerate()
            .map(|(i, pixels)| Layer::new(pixels.to_vec(), i, shape))
            .collect::<Vec<_>>();
        Self { layers, shape }
    }

    fn decode(&self) -> Layer {
        let mut image = Layer::new(vec![2; self.shape.0 * self.shape.1], 0, self.shape);
        for layer in &self.layers {
            for y in 0..layer.get_height() {
                for x in 0..layer.get_width() {
                    let value = layer.get(x, y);
                    if image.get(x, y) == 2 && value != 2 {
                        image.set(x, y, value);
                    }
                }
            }
        }
        image
    }
}

fn main() {
    let input_file = std::env::args().nth(1).expect("Usage: day8 INPUT_FILE");
    let input = std::fs::read_to_string(input_file).expect("Error reading input file");
    let mut image = Image::new(&input, (25, 6));

    // Part 1
    let layer = image
        .layers
        .iter()
        .min_by_key(|layer| layer.count(0))
        .unwrap();

    println!("Part 1: Layer #{} => Count of (1) * (2) {}", layer.get_id(), layer.count(1) * layer.count(2));

    // Part 2
    let image = image.decode();
    println!("Part 2:");
    for y in 0..image.get_height() {
        for x in 0..image.get_width() {
            let value = image.get(x, y);
            print!("{}", if value == 1 { '#' } else { ' ' });
        }
        println!();
    }
}
