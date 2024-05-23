use {
    crate::config::{
        ConfigOptions,
        ProviderConfigOptions,
        RandomnessOptions,
    },
    anyhow::Result,
    clap::Args,
    std::fs,
};

#[derive(Args, Clone, Debug)]
#[command(next_help_heading = "Setup Provider Options")]
#[group(id = "SetupProviderOptions")]
pub struct SetupProviderOptions {
    #[command(flatten)]
    pub config: ConfigOptions,

    #[command(flatten)]
    pub provider_config: ProviderConfigOptions,

    /// Path to a file containing a 20-byte (40 char) hex encoded Ethereum private key.
    /// This key is required to submit transactions (such as registering with the contract).
    #[arg(long = "private-key")]
    #[arg(env = "PRIVATE_KEY")]
    pub private_key_file: String,

    #[command(flatten)]
    pub randomness: RandomnessOptions,

    /// The base URI for fortuna.
    /// e.g., https://fortuna-staging.dourolabs.app
    #[arg(long = "uri")]
    pub base_uri: String,
}

impl SetupProviderOptions {
    pub fn load_private_key(&self) -> Result<String> {
        return Ok((fs::read_to_string(&self.private_key_file))?);
    }
}
