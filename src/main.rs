use std::process::Command;

fn normalize_audio(input_path: &str, output_path: &str) {
    let output = if is_audio_file(input_path) {
        Command::new("ffmpeg")
            .args(&[
                "-i",
                input_path,
                "-af",
                "loudnorm=I=-23:LRA=7:TP=-2:measured_I=-23:measured_LRA=7:measured_TP=-2:measured_thresh=-37.5:offset=0.0:linear=true:print_format=json",
                "-c:a",
                "copy",
                output_path,
            ])
            .output()
            .expect("Failed to execute FFmpeg")
    } else {
        Command::new("ffmpeg")
            .args(&[
                "-i",
                input_path,
                "-af",
                "loudnorm=I=-23:LRA=7:TP=-2:measured_I=-23:measured_LRA=7:measured_TP=-2:measured_thresh=-37.5:offset=0.0:linear=true:print_format=json",
                "-c:v",
                "copy",
                "-c:a",
                "aac",
                output_path,
            ])
            .output()
            .expect("Failed to execute FFmpeg")
    };

    if output.status.success() {
        println!("Audio normalization completed successfully!");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error: {}", stderr);
    }
}

fn is_audio_file(file_path: &str) -> bool {
    let audio_extensions = ["mp3", "wav", "aac"]; // Add more audio extensions if needed
    let extension = file_path
        .split('.')
        .last()
        .map(|ext| ext.to_lowercase());

    if let Some(ext) = extension {
        audio_extensions.contains(&ext.as_str())
    } else {
        false
    }
}

fn main() {
    let input_path = "input.mp4";
    let output_path = "output_normalized.mp4";

    normalize_audio(input_path, output_path);
}