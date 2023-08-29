use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "vscode-extension-ids", author, version, about, long_about = None)]
pub struct ExtensionsArgs {
        /// Path to the file containing installed extensions
        #[arg(short, long)]
        pub path: String,
    
        /// Wether to prefix the output with "@id:" for searching the extension in the extensions marketplace search
        #[arg(short, long)]
        pub with_prefix: bool,

        /// Save the output to a file
        #[arg(short, long)]
        pub output_path: Option<String>,
}
