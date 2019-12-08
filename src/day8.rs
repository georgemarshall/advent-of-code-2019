use ansi_term::Color::{Black, White};
use itertools::Itertools;

fn image_layers(image: &[u8], width: usize, height: usize) -> Vec<Vec<u8>> {
    image.chunks(width * height).map(|c| c.to_owned()).collect()
}

#[aoc_generator(day8)]
fn load_image(input: &str) -> Vec<u8> {
    input
        .lines()
        .map(|s| {
            s.chars()
                .filter_map(|c| c.to_string().parse().ok())
                .collect()
        })
        .exactly_one()
        .unwrap()
}

#[aoc(day8, part1)]
fn image_checksum(image: &[u8]) -> Option<u32> {
    image_layers(image, 25, 6)
        .into_iter()
        .map(|layer| {
            layer.into_iter().fold([0; 3], |mut acc, pixel| {
                acc[pixel as usize] += 1;
                acc
            })
        })
        .min_by(|a, b| a[0].cmp(&b[0]))
        .map(|c| c[1] * c[2])
}

#[aoc(day8, part2)]
fn image_decode(image: &[u8]) -> String {
    let composite_image =
        image_layers(image, 25, 6)
            .into_iter()
            .fold([2; 25 * 6], |mut acc, layer| {
                for (i, pixel) in layer.into_iter().enumerate() {
                    if acc[i] == 2 {
                        acc[i] = pixel;
                    }
                }
                acc
            });
    let rendered_output = composite_image
        .into_iter()
        .map(|pixel| match pixel {
            0 => Black.paint("█").to_string(),
            1 => White.paint("█").to_string(),
            _ => unreachable!(),
        })
        .chunks(25)
        .into_iter()
        .map(|r| {
            let mut line = String::from("\t");
            line.extend(r.into_iter());
            line
        })
        .join("\n");

    let mut output = String::from("\n\n");
    output.push_str(&rendered_output);
    output.push('\n');
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_image() {
        let input = "123456789012\n";

        assert_eq!(load_image(input), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
    }

    #[test]
    fn test_process() {
        let image = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2];
        assert_eq!(
            image_layers(&image, 3, 2),
            vec![vec![1, 2, 3, 4, 5, 6], vec![7, 8, 9, 0, 1, 2],]
        );
    }
}
