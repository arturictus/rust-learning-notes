struct Image<P> {
    width: usize,
    pixels: Vec<P>,
}

impl<P: Default + Copy> Image<P> {
    /// Create a new image of the given size.
    fn new(width: usize, height: usize) -> Image<P> {
        Image {
            width,
            pixels: vec![P::default(); width * height],
        }
    }
}

impl<P> std::ops::Index<usize> for Image<P> {
    type Output = [P];
    fn index(&self, row: usize) -> &[P] {
        let start = row * self.width;
        &self.pixels[start..start + self.width]
    }
}

impl<P> std::ops::IndexMut<usize> for Image<P> {
    // Returns the row with all the elements, each element
    // represents a column
    fn index_mut(&mut self, row: usize) -> &mut [P] {
        let start = row * self.width;
        &mut self.pixels[start..start + self.width]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_image_indexes() {
        let mut image: Image<u32> = Image::new(13, 13);
        let row = 10;
        let column = 0;
        assert_eq!(image[row][column], 0);
        image[12][0] = 1;
        image.index_mut(12)[2] = 2;
        assert_eq!(image[12][0], 1);
        assert_eq!(image[12][2], 2);
    }

    // ----------------
    // Index examples with HashMap and Vec
    // ------------------

    use std::{collections::HashMap, ops::IndexMut};

    #[test]
    fn test_hash_map_index() {
        let mut m = HashMap::new();
        m.insert("十", 10);
        m.insert("百", 100);
        m.insert("千", 1000);
        m.insert("万", 1_0000);
        m.insert("億", 1_0000_0000);

        assert_eq!(m["十"], 10);
        assert_eq!(m["千"], 1000);

        use std::ops::Index;
        assert_eq!(*m.index("十"), 10);
        assert_eq!(*m.index("千"), 1000);
    }

    #[test]
    fn test_vec_index() {
        let mut desserts = vec!["Howalon".to_string(), "Soan papdi".to_string()];
        desserts[0].push_str(" (fictional)");
        desserts[1].push_str(" (real)");
        use std::ops::IndexMut;
        (*desserts.index_mut(0)).push_str(" (fictional)");
        (*desserts.index_mut(1)).push_str(" (real)");
    }
}
