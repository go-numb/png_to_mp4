use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use clap::Parser;

fn convert_images_to_video(
    input_folder: &Path,
    output_file: &Path,
    exe_type: &str,
) -> Result<(), Box<dyn Error>> {
    // 画像の表示時間
    let duration_per_image = 5;

    // 対象ファイル郡
    let files = format!("%d.{}", exe_type);

    let cmd = Command::new("ffmpeg")
        .arg("-framerate")
        .arg(format!("1/{}", duration_per_image))
        .arg("-i")
        .arg(input_folder.join(files))
        .arg("-c:v")
        .arg("h264_nvenc")
        .arg("-pix_fmt")
        .arg("yuv420p")
        .arg("-rc")
        .arg("vbr")
        .arg("-preset")
        .arg("slow")
        .arg("-b:v")
        .arg("5M")
        .arg("-maxrate")
        .arg("10M")
        .arg("-bufsize")
        .arg("15M")
        .arg("-profile:v")
        .arg("high")
        .arg("-crf")
        .arg("14")
        .arg("-vf")
        .arg("fps=2")
        .arg("-y")
        .arg(output_file)
        .output()?;

    if cmd.status.success() {
        println!("変換成功: {:?}", output_file);
        Ok(())
    } else {
        let error = String::from_utf8_lossy(&cmd.stderr);
        Err(format!("変換失敗 {:?}: {}", input_folder, error).into())
    }
}

fn get_subdirectories(directory: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    Ok(fs::read_dir(directory)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_dir() {
                Some(path)
            } else {
                None
            }
        })
        .collect())
}

fn process_directories(
    parent_dir: &Path,
    output_dir: &Path,
    skip_dir: &str,
    target_ext: &str,
) -> Result<(), Box<dyn Error>> {
    let subdirs = get_subdirectories(parent_dir)?;

    for subdir in subdirs {
        if subdir
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .contains(skip_dir)
        {
            println!(
                "スキップ: {:?} (キーワード '{}' を含むため)",
                subdir, skip_dir
            );
            continue;
        }

        let filename = truncate_filename(subdir.file_name().unwrap().to_str().unwrap(), 24);
        let output_file = output_dir.join(format!("{}.mp4", filename));

        fs::create_dir_all(output_dir)?;

        convert_images_to_video(&subdir, &output_file, target_ext)?;
    }

    Ok(())
}

fn truncate_filename(filename: &str, max_length: usize) -> String {
    if filename.len() > max_length {
        filename[..max_length].to_string()
    } else {
        filename.to_string()
    }
}

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// 親ディレクトリのパス
    #[clap(short, long, value_parser, default_value = ".", value_name = "DIR")]
    parent: String,

    /// 親ディレクトリ内の出力ディレクトリの名称
    #[clap(
        short,
        long,
        value_parser,
        default_value = "output",
        value_name = "DIR"
    )]
    output: String,

    /// 連番の画像ファイルの拡張子
    /// 例: jpg, png, bmp
    #[clap(short, long, value_parser, default_value = "png", value_name = "EXE")]
    target: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    println!("親ディレクトリ: {:?}", args.parent);
    println!("出力ディレクトリ: {:?}", args.output);

    let parent = Path::new(args.parent.as_str());
    let output = Path::new(args.output.as_str());
    let output_dir = parent.join(output);
    let target_ext = args.target.as_str();

    println!("出力ディレクトリ: {:?}", output_dir);

    process_directories(parent, &output_dir, output.to_str().unwrap(), target_ext)?;

    Ok(())
}
