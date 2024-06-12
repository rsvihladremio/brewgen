use clap::Parser;
use reqwest::Url;
use sha256::try_digest;
use std::fs::{self, File};
use std::{error::Error, io, path::Path};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    repo: String,

    #[arg(short, long)]
    owner: String,

    #[arg(short, long)]
    desc: String,

    #[arg(short, long)]
    binary: String,

    #[arg(short, long)]
    test_command: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let octocrab = octocrab::instance();
    let repo = octocrab.repos(args.owner.clone(), args.repo.clone());
    let releases = repo.releases();
    let latest = releases.get_latest().await;
    let release = latest.unwrap();
    let mut linux_amd64_url = "".to_string();
    let mut linux_amd64_sha = "".to_string();
    let mut linux_arm64_url = "".to_string();
    let mut linux_arm64_sha = "".to_string();
    let mut mac_amd64_url = "".to_string();
    let mut mac_amd64_sha = "".to_string();
    let mut mac_arm64_url = "".to_string();
    let mut mac_arm64_sha = "".to_string();

    for asset in release.clone().assets {
        let url = asset.clone().browser_download_url;
        let name = asset.clone().name;
        let name_lower = name.to_lowercase();
        if name_lower.contains("amd64") && name_lower.contains("linux") {
            linux_amd64_url = url.to_string();
            download_file(&url, &name).await;
            linux_amd64_sha = sha_file(&asset.name.clone()).unwrap();
        } else if name_lower.contains("arm64") && name_lower.contains("linux") {
            linux_arm64_url = url.to_string();
            download_file(&url, &name).await;
            linux_arm64_sha = sha_file(&asset.name.clone()).unwrap();
        } else if name_lower.contains("mac") || name.contains("darwin") {
            if name_lower.contains("arm")
                || name_lower.contains("m-series")
                || name_lower.contains("silicon")
            {
                mac_arm64_url = url.to_string();
                download_file(&url, &name).await;
                mac_arm64_sha = sha_file(&asset.name.clone()).unwrap();
            } else {
                mac_amd64_url = url.to_string();
                download_file(&url, &name).await;
                mac_amd64_sha = sha_file(&asset.name.clone()).unwrap();
            }
        }
    }
    let mut v: Vec<char> = args.binary.chars().collect();
    v[0] = v[0].to_uppercase().nth(0).unwrap();
    let s2: String = v.into_iter().collect();
    let binary_title: &String = &s2;
    println!(
        "class {} < Formula
  desc \"{}\"
  homepage \"https://github.com/{}/{}\"
  version \"{}\"

  on_macos do
    if Hardware::CPU.intel?
      # Define the download URLs and corresponding SHA256 checksums for the binary releases
      url \"{}\"
      sha256 \"{}\"
    end

    if Hardware::CPU.arm?
      url \"{}\"
      sha256 \"{}\"
    end
  end

  on_linux do
    if Hardware::CPU.intel?
      # Define the download URLs and corresponding SHA256 checksums for the binary releases
      url \"{}\"
      sha256 \"{}\"
    end

    if Hardware::CPU.arm? && Hardware::CPU.is_64_bit?
      url \"{}\"
      sha256 \"{}\"
    end
  end

  def install
    libexec.install \"bin/{}\" => \"{}\"
    bin.write_exec_script libexec/\"{}\"
  end

  test do
    # Add test logic here if applicable
    system \"#{{bin}}/{}\", \"{}\"
  end
end",
        binary_title,
        args.desc,
        args.owner,
        args.repo,
        release.clone().tag_name.trim_start_matches("v"),
        mac_amd64_url,
        mac_amd64_sha,
        mac_arm64_url,
        mac_arm64_sha,
        linux_amd64_url,
        linux_amd64_sha,
        linux_arm64_url,
        linux_arm64_sha,
        args.binary,
        args.binary,
        args.binary,
        args.binary,
        args.test_command,
    );
}

async fn download_file(url: &Url, file_name: &String) {
    let bytes_future = reqwest::get(url.to_string()).await.unwrap().bytes();
    let bytes = bytes_future.await.unwrap();
    let mut resp = bytes.as_ref();
    let mut out = File::create(file_name).unwrap();
    io::copy(&mut resp, &mut out).expect("failed to copy content");
}

fn sha_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    //sha256 digest file
    let input = Path::new(file_path);
    let val = try_digest(input)?;
    fs::remove_file(file_path)?;
    Ok(val)
}
