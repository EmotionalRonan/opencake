//! Model slot registry (design doc §5). Skeleton: types only.

use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Slot {
    FaceDetect, FaceLandmark, FaceParse, HumanParse, BodyPose,
    SkinRetouch, Blemish, Inpaint, Matting, InteractiveSelect,
    SkySeg, Upscale, FaceRestore, ColorAuto, ColorMatch, FaceEmbed,
    Cull, Aigc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LicenseTier { Permissive, Copyleft, Openrail, DataNc, NonCommercial }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Backend { LocalOnnx, Comfyui, Http }

#[derive(Debug, Deserialize)]
pub struct License { pub code: String, pub weights: String, pub tier: LicenseTier, pub requires_ack: bool }

#[derive(Debug, Deserialize)]
pub struct Runtime { pub backend: Backend, #[serde(default)] pub ep_preference: Vec<String> }

#[derive(Debug, Deserialize)]
pub struct ModelCard {
    pub schema: String,
    pub id: String,
    pub slot: Slot,
    pub license: License,
    pub runtime: Runtime,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid model card: {0}")]
    Parse(#[from] serde_yaml::Error),
}

pub fn parse_model_card(yaml: &str) -> Result<ModelCard, Error> {
    Ok(serde_yaml::from_str(yaml)?)
}
