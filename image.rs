#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pixel {
   pub r: u8, pub g: u8, pub b: u8, 
}

impl Pixel {

    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Pixel { r, g, b }
    }
}

impl std::fmt::Display for Pixel{
  fn fmt(&self, f: &mut std::fmt::Formatter)-> std::fmt::Result{
    write!(f, "{} {} {}", self.r, self.g, self.b)
  }
}

// implement the Image struct and traits below

pub struct Image{
  pub width:usize, pub height:usize, data:Vec<Pixel>,
}

impl Image{
  pub fn new(width:usize, height:usize)-> Image{
      let data = vec![Pixel{r: 0, g: 0, b: 0}; width * height];
      Image{width, height, data}
  }

  pub fn get(&self, x:usize, y:usize)-> Option<&Pixel>{
    if x < self.width && y < self.height{
      let index = y * self.width + x;
      Some(&self.data[index])
    }else{
      None
      }
  }

      pub fn get_mut(&mut self, x:usize, y:usize)-> Option<&mut Pixel>{
    if x < self.width && y < self.height{
      let index = y * self.width + x;
      Some(&mut self.data[index])
    }else{
      None
      }

  }

    pub fn get_mandelbrot_pixels(&self) -> usize{
    self.data.iter().filter(|&pixl| {
      pixl.r == 0 && pixl.g == 0 && pixl.b == 0 
      }).count()
  }
}