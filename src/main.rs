use app::App;

mod app;
mod diagram;
mod tui;

fn main() -> anyhow::Result<()> {
    #[cfg(not(debug_assertions))]
    sudo::escalate_if_needed()
        .map_err(|_| anyhow::anyhow!("unable to escalate to root privilege"))?;

    let mut terminal = tui::init()?;
    let result = App::default().run(&mut terminal);
    tui::restore()?;
    result
}
