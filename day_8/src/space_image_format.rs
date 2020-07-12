pub struct SpaceImageFormat {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl SpaceImageFormat {
    pub fn layer_size(&self) -> usize {
        self.height * self.width
    }

    pub fn layer_count(&self) -> usize {
        self.data.len() / self.layer_size()
    }

    pub fn parse(source: &str, width: usize, height: usize) -> SpaceImageFormat {
        let data = source
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        SpaceImageFormat {
            data,
            width,
            height,
        }
    }

    pub fn get_layer(&self, index: usize) -> Result<&[u8], ()> {
        if index >= self.layer_count() {
            return Err(());
        }

        let layer_size = self.layer_size();
        let start = layer_size * index;
        Ok(&self.data[start..start + layer_size])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_IMAGE: &str = "123456789012";

    #[test]
    fn basic_layer_count() {
        let image = SpaceImageFormat::parse(TEST_IMAGE, 3, 2);
        assert_eq!(image.layer_count(), 2);
    }

    #[test]
    fn get_layer_in_range() {
        let image = SpaceImageFormat::parse(TEST_IMAGE, 3, 2);
        assert!(image.get_layer(0).is_ok());
    }

    #[test]
    fn get_layer_out_range() {
        let image = SpaceImageFormat::parse(TEST_IMAGE, 3, 2);
        assert!(image.get_layer(2).is_err());
    }
}
