use uuid::Uuid;
use zed_extension_api::SlashCommandOutput;

pub fn generate_uuid() -> Result<SlashCommandOutput, String> {
    let id = Uuid::new_v4();
    let id_string = id.to_string();

    Ok(SlashCommandOutput {
        sections: vec![zed_extension_api::SlashCommandOutputSection {
            range: (0..id_string.len()).into(),
            label: id_string.to_string(),
        }],
        text: id_string,
    })
}
