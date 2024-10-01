# 連番画像から動画作成ツール

## 概要

このツールは、指定されたディレクトリ内の画像ファイルを使用して動画を作成します。複数のサブディレクトリを処理し、各サブディレクトリの画像からディレクトリごとの動画ファイルを生成します。動画ファイル名はディレクトリ名です。

## 特徴

- 複数のサブディレクトリを一括処理
- FFmpegを使用した高品質な動画エンコーディング
- GPUアクセラレーション対応（NVIDIA GPUが必要）
- カスタマイズ可能なフレームレートと画質設定
- 柔軟なコマンドラインインターフェース

## 必要条件

- FFmpeg（システムにインストールされ、PATHに含まれていること）
- NVIDIA GPU（GPUエンコーディングを使用する場合）

## インストール

1. このリポジトリをクローンします：

git clone https://github.com/png_to_mp4/image-to-video-tool.git


2. プロジェクトディレクトリに移動します：

cd image-to-video-tool


3. Rustの依存関係をインストールします：

cargo build --release

4. 適宜パスを通してください



## 使用方法

基本的な使用方法：


png_to_mp4 -p <対象ディレクトリのパス> -o <出力ディレクトリのパス>



オプション：
- `-p, --parent <DIR>`: 画像が含まれる親ディレクトリのパス（デフォルト: "."）
- `-o, --output <DIR>`: 生成された動画ファイルを保存するディレクトリのパス（デフォルト: "output"）
- `-t, --target <EXE>`: 対象画像連番ファイルの拡張子（デフォルト: "png"）

例：

png_to_mp4 -p C:\Users\YourName\Pictures -o C:\Users\YourName\Videos -s jpg



## 注意事項

- 大量の画像ファイルを処理する場合、十分なディスク容量があることを確認してください。
- GPUエンコーディングを使用する場合、互換性のあるNVIDIA GPUとドライバーが必要です。

## Author

[@_numbP](https://twitter.com/_numbP)

## License

[MIT](https://github.com/go-numb/png_to_mp4/blob/master/LICENSE)