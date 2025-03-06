mod commands;
mod utils;

use commands::{encoding, hashing};
use utils::{uuid_gen, xor};

use zed_extension_api::{
    self as zed, SlashCommand, SlashCommandArgumentCompletion, SlashCommandOutput, Worktree,
};

struct CyberZedExtension;

impl zed::Extension for CyberZedExtension {
    fn new() -> Self {
        CyberZedExtension
    }

    fn complete_slash_command_argument(
        &self,
        command: SlashCommand,
        _args: Vec<String>,
    ) -> Result<Vec<zed::SlashCommandArgumentCompletion>, String> {
        match command.name.as_str() {
            "echo" => Ok(vec![]),
            "base64" | "hex" | "url" => encoding::complete_encoding_commands(),
            "sha" => hashing::complete_sha_commands(),
            "md5" => hashing::complete_md5_commands(),
            "uuid" => Ok(vec![SlashCommandArgumentCompletion {
                label: "uuid".to_string(),
                new_text: "uuid".to_string(),
                run_command: true,
            }]),
            "xor" => Ok(vec![SlashCommandArgumentCompletion {
                label: "xor".to_string(),
                new_text: "xor".to_string(),
                run_command: true,
            }]),
            _ => Err(format!("Unknown slash command: \"{}\"", command.name)),
        }
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        args: Vec<String>,
        _worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        match command.name.as_str() {
            // "echo" => echo_command(&args),
            "uuid" => uuid_gen::generate_uuid(),
            "xor" => xor::xor_command(args),
            "base64" => encoding::process_encoding_command(
                "Base64",
                &args,
                encoding::encode_base64,
                encoding::decode_base64,
            ),
            "hex" => encoding::process_encoding_command(
                "Hex",
                &args,
                encoding::encode_hex,
                encoding::decode_hex,
            ),
            "url" => encoding::process_encoding_command(
                "URL",
                &args,
                encoding::encode_url,
                encoding::decode_url,
            ),
            "sha" => hashing::process_sha_command(&args),
            "md5" => hashing::process_md5_command(&args),
            _ => Err(format!("Unknown slash command: \"{}\"", command.name)),
        }
    }
}

zed::register_extension!(CyberZedExtension);
