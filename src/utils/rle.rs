pub fn rle_encode(input: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();

    if input.is_empty() {
        return output;
    }

    let mut current = input[0];
    let mut count: u8 = 1;

    for &byte in &input[1..] {
        if byte == current && count < 255 {
            count += 1;
        } else {
            output.push(count);
            output.push(current);
            current = byte;
            count = 1;
        }
    }

    output.push(count);
    output.push(current);

    output
}
