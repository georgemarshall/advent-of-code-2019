use ansi_term::Color;
use itertools::Itertools;

const IMG_W: usize = 25;
const IMG_H: usize = 6;
const PIXEL: &str = "█";

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
    image_layers(image, IMG_W, IMG_H)
        .into_iter()
        .map(|layer| {
            layer.into_iter().fold([0; 10], |mut acc, pixel| {
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
        image_layers(image, IMG_W, IMG_H)
            .into_iter()
            .fold([2; IMG_W * IMG_H], |mut acc, layer| {
                for (i, pixel) in layer.into_iter().enumerate() {
                    if acc[i] == 2 {
                        acc[i] = pixel;
                    }
                }
                acc
            });

    let mut output = String::from("\n\n");
    let output_lines = composite_image
        .iter()
        .map(|pixel| {
            match pixel {
                0 => Color::Black,
                1 => Color::White,
                _ => unreachable!(),
            }
            .paint(PIXEL)
            .to_string()
        })
        .chunks(IMG_W);

    output.extend(output_lines.into_iter().map(|row| {
        let mut line = String::from("\t");
        line.extend(row);
        line.push('\n');
        line
    }));
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
