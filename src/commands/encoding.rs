use base64::prelude::*;
use hex;
use urlencoding;
use zed_extension_api::{self as zed, SlashCommandArgumentCompletion};

pub fn complete_encoding_commands() -> Result<Vec<zed::SlashCommandArgumentCompletion>, String> {
    Ok(vec![
        SlashCommandArgumentCompletion {
            label: "encode".to_string(),
            new_text: "encode".to_string(),
            run_command: true,
        },
        SlashCommandArgumentCompletion {
            label: "decode".to_string(),
            new_text: "decode".to_string(),
            run_command: true,
        },
    ])
}

pub fn process_encoding_command<F, G>(
    _: &str,
    args: &[String],
    encode: F,
    decode: G,
) -> Result<zed::SlashCommandOutput, String>
where
    F: Fn(&[u8]) -> Result<String, String>,
    G: Fn(&str) -> Result<String, String>,
{
    let Some(selection) = args.first() else {
        return Err("No option selected".to_string());
    };

    if args.len() != 2 {
        return Err("Invalid number of arguments".to_string());
    }
    if args[1].is_empty() {
        return Err("Empty input".to_string());
    }

    let input = args[1].to_string();
    let output = match selection.as_str() {
        "encode" => encode(input.as_bytes())?,
        "decode" => decode(&input)?,
        _ => return Err(format!("{} is not a valid option", selection)),
    };

    let label = if output.len() <= 32 {
        output.clone()
    } else {
        format!("{}...", &output[0..32])
    };

    Ok(zed::SlashCommandOutput {
        sections: vec![zed::SlashCommandOutputSection {
            range: (0..output.len()).into(),
            label: label,
        }],
        text: output,
    })
}

pub fn encode_base64(input: &[u8]) -> Result<String, String> {
    Ok(BASE64_STANDARD.encode(input))
}

pub fn decode_base64(input: &str) -> Result<String, String> {
    BASE64_STANDARD
        .decode(input)
        .map_err(|_| "Invalid Base64 input".to_string())
        .and_then(|bytes| {
            String::from_utf8(bytes).map_err(|_| "Decoded bytes are not valid UTF-8".to_string())
        })
}

pub fn encode_hex(input: &[u8]) -> Result<String, String> {
    Ok(hex::encode(input))
}

pub fn decode_hex(input: &str) -> Result<String, String> {
    hex::decode(input)
        .map_err(|_| "Invalid Hex input".to_string())
        .and_then(|bytes| {
            String::from_utf8(bytes).map_err(|_| "Decoded bytes are not valid UTF-8".to_string())
        })
}

pub fn encode_url(input: &[u8]) -> Result<String, String> {
    Ok(urlencoding::encode(&(String::from_utf8_lossy(input).to_string())).to_string())
}

pub fn decode_url(input: &str) -> Result<String, String> {
    urlencoding::decode(input)
        .map(|decoded| decoded.into_owned()) // Converte `Cow<str>` in `String`
        .map_err(|_| "Invalid URL input".to_string())
}
