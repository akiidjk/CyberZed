use hex;
use zed_extension_api::SlashCommandOutput;

fn xor_bytes(a: &[u8], b: &[u8]) -> Result<Vec<u8>, String> {
    let min_len = a.len().min(b.len());
    let result: Vec<u8> = a
        .iter()
        .take(min_len)
        .zip(b.iter().take(min_len))
        .map(|(&x, &y)| x ^ y)
        .collect();

    Ok(result)
}

fn is_hex_valid(input: &str) -> bool {
    hex::decode(input).is_ok()
}

pub fn xor_command(args: Vec<String>) -> Result<SlashCommandOutput, String> {
    if args.len() < 2 {
        return Err("Not enough arguments. Expected two strings to XOR.".to_string());
    }

    let op1 = args[0].clone();
    let op2 = args[1].clone();

    match xor_bytes(op1.as_bytes(), op2.as_bytes()) {
        Ok(result) => {
            let hex_result = hex::encode(result);

            if !is_hex_valid(&hex_result) {
                return Err("Output is not a valid hex string".to_string());
            }

            let label = if hex_result.len() <= 32 {
                hex_result.clone()
            } else {
                format!("{}...", &hex_result[0..32])
            };

            Ok(SlashCommandOutput {
                sections: vec![zed_extension_api::SlashCommandOutputSection {
                    range: (0..hex_result.len()).into(),
                    label,
                }],
                text: hex_result,
            })
        }
        Err(err) => Err(err),
    }
}
