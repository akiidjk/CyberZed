use md5;
use sha2::{Digest, Sha256, Sha384, Sha512};
use zed_extension_api::{self as zed, SlashCommandArgumentCompletion};

pub fn complete_sha_commands() -> Result<Vec<zed::SlashCommandArgumentCompletion>, String> {
    Ok(vec![
        SlashCommandArgumentCompletion {
            label: "256".to_string(),
            new_text: "sha256".to_string(),
            run_command: true,
        },
        SlashCommandArgumentCompletion {
            label: "384".to_string(),
            new_text: "sha384".to_string(),
            run_command: true,
        },
        SlashCommandArgumentCompletion {
            label: "512".to_string(),
            new_text: "sha512".to_string(),
            run_command: true,
        },
    ])
}

pub fn process_sha_command(args: &[String]) -> Result<zed::SlashCommandOutput, String> {
    let Some(selection) = args.first() else {
        return Err("No option selected".to_string());
    };

    let input = args[1..].join(" ");
    let output = match selection.as_str() {
        "256" => {
            let mut hasher = Sha256::new();
            hasher.update(input.as_bytes());
            hex::encode(hasher.finalize())
        }
        "384" => {
            let mut hasher = Sha384::new();
            hasher.update(input.as_bytes());
            hex::encode(hasher.finalize())
        }
        "512" => {
            let mut hasher = Sha512::new();
            hasher.update(input.as_bytes());
            hex::encode(hasher.finalize())
        }
        _ => return Err("Invalid SHA hash length".to_string()),
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

pub fn complete_md5_commands() -> Result<Vec<zed::SlashCommandArgumentCompletion>, String> {
    Ok(vec![SlashCommandArgumentCompletion {
        label: "md5".to_string(),
        new_text: "md5".to_string(),
        run_command: true,
    }])
}

pub fn process_md5_command(args: &[String]) -> Result<zed::SlashCommandOutput, String> {
    let input = args.join(" ");
    let digest = md5::compute(input.as_bytes());
    let output = hex::encode(*digest);

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
