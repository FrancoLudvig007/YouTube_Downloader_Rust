mod pyrsutils;
use pyrsutils::prelude::*;
use std::process::{Command, Stdio};

fn disponivel(programa: &str) -> bool {
    Command::new(programa)
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn ler(prompt: &str) -> Option<String> {
    match try_input::<String>(prompt) {
        Ok(texto) => Some(strip(&texto).to_string()),
        Err(_) => None,
    }
}

fn baixar(url: &str, pasta: &str, audio: bool) -> bool {
    if let Err(e) = makedirs(pasta) {
        print(format!("Erro ao criar a pasta '{pasta}': {e}"));
        return false;
    }

    let modelo = join_path(&[pasta, "%(title)s.%(ext)s"]);
    let mut cmd = Command::new("yt-dlp");
    cmd.args(["--no-playlist", "--newline", "-o", modelo.as_str()]);

    if audio {
        cmd.args(["-x", "--audio-format", "mp3", "--audio-quality", "0"]);
    } else {
        cmd.args([
            "-f",
            "bv*[ext=mp4]+ba[ext=m4a]/b[ext=mp4]/bv*+ba/b",
            "--merge-output-format",
            "mp4",
        ]);
    }
    cmd.arg("--").arg(url);

    match cmd.status() {
        Ok(status) => status.success(),
        Err(e) => {
            print(format!("Falha ao executar o yt-dlp: {e}"));
            false
        }
    }
}

fn main() {
    print("=== YouTube Downloader (MP4 / MP3) ===");

    if !disponivel("yt-dlp") {
        print("yt-dlp não encontrado no PATH. Instale-o e tente novamente.");
        return;
    }
    if !disponivel("ffmpeg") {
        print("Aviso: ffmpeg não encontrado. A conversão para MP3 e a junção de vídeo+áudio falharão.");
    }

    loop {
        print("\n[1] Baixar vídeo (.mp4)");
        print("[2] Baixar música (.mp3)");
        print("[0] Sair");

        let opcao = match ler("Opção: ") {
            Some(o) => o,
            None => break,
        };

        let audio = match opcao.as_str() {
            "0" => break,
            "1" => false,
            "2" => true,
            _ => {
                print("Opção inválida.");
                continue;
            }
        };

        let url = match ler("URL do vídeo: ") {
            Some(u) => u,
            None => break,
        };
        if len(url.as_str()) == 0 || !startswith(&lower(&url), "http") {
            print("URL inválida.");
            continue;
        }

        let pasta = match ler("Pasta de destino [downloads]: ") {
            Some(p) if len(p.as_str()) > 0 => p,
            Some(_) => String::from("downloads"),
            None => break,
        };

        print("Baixando...");
        if baixar(&url, &pasta, audio) {
            print(format!("Concluído! Arquivo salvo em '{pasta}'."));
        } else {
            print("O download falhou.");
        }
    }

    print("Até logo!");
}