use dalamud;
use sysinfo::{ProcessExt, SystemExt};
use tracing::Level;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::test]
async fn dalamud_starts() {
    tracing_subscriber::fmt::fmt()
        .with_max_level(Level::DEBUG)
        .init();
    let dalamud_root = std::path::Path::new("./testing/dalamud_root");
    // let _ = std::fs::remove_dir_all(dalamud_root);
    // assert!(!dalamud_root.exists());
    std::fs::create_dir_all(dalamud_root).unwrap();
    let settings = dalamud::Settings::default();
    // TODO (Chiv) Split tests and such and make this portable
    let game = std::path::Path::new(r"C:\Games\SquareEnix\FINAL FANTASY XIV - A Realm Reborn\");
    let ffxiv = start_dev_ffxiv().unwrap();
    dalamud::download_and_start(
        dalamud::RootPath(dalamud_root),
        dalamud::GamePath(game),
        libxl::game::language::ClientLanguage::English,
        settings,
        ffxiv.id() as usize,
    )
    .await
    .unwrap();
    //let _ = std::fs::remove_dir_all(dalamud_root);
}

fn start_dev_ffxiv() -> Result<ffxiv::native::Process, Box<dyn std::error::Error>> {
    let mut s = sysinfo::System::new();
    s.refresh_processes();
    let ffxiv = s
        .processes()
        .into_iter()
        .find(|(pid, p)| p.name() == "ffxiv_dx11.exe");
    assert!(ffxiv.is_none());
    // TODO (Chiv) Split tests and such and make this portable
    let game = std::path::Path::new(r"C:\Games\SquareEnix\FINAL FANTASY XIV - A Realm Reborn\");
    //TODO  DEV ARGS Start Args DEV.DataPathType=1 DEV.MaxEntitledExpansionID=3 DEV.TestSID=a DEV.UseSqPack=1 SYS.Region=0 language=1
    ffxiv::launch(
        ffxiv::SessionId("a"),
        ffxiv::Region(0),
        ffxiv::ExpansionLevel(3),
        ffxiv::SteamIntegration::No,
        ffxiv::SteamServiceAccount::No,
        None,
        game,
        ffxiv::DX11::Yes,
        libxl::game::language::ClientLanguage::English,
        ffxiv::EncryptArguments::No,
        ffxiv::FfxivVersion(""),
    )
}
