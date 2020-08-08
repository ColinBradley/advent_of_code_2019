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

    pub fn decode(&self) -> String {
        let layer_count = self.layer_count();
        let layer_size = self.layer_size();

        let mut result = Vec::<String>::new();

        for pixel_index in 0..layer_size {
            result.push((|| {
                for layer_index in 0..layer_count {
                    let pixel_data = self.data[layer_index * layer_size + pixel_index];
                    if pixel_data == 2 {
                        continue;
                    }

                    return pixel_data.to_string();
                }

                String::from("2")
            })());
        }

        let mut lines = result
            .rchunks(self.width)
            .map(|line| line.join(""))
            .collect::<Vec<String>>();

        lines.reverse();

        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_IMAGE_DATA_1: &str = "123456789012";

    #[test]
    fn basic_layer_count() {
        let image = SpaceImageFormat::parse(TEST_IMAGE_DATA_1, 3, 2);
        assert_eq!(image.layer_count(), 2);
    }

    #[test]
    fn get_layer_in_range() {
        let image = SpaceImageFormat::parse(TEST_IMAGE_DATA_1, 3, 2);
        assert!(image.get_layer(0).is_ok());
    }

    #[test]
    fn get_layer_out_range() {
        let image = SpaceImageFormat::parse(TEST_IMAGE_DATA_1, 3, 2);
        assert!(image.get_layer(2).is_err());
    }

    const TEST_IMAGE_DATA_2: &str = "0222112222120000";

    #[test]
    fn decode() {
        let image = SpaceImageFormat::parse(TEST_IMAGE_DATA_2, 2, 2);
        assert_eq!(image.decode(), "01\n10");
    }
}
