use dita_design_system::hooks::header::HeaderMode;
use dita_design_system::hooks::sidenav::SidenavMode;
use dita_design_system::hooks::theme::ThemeMode;
use dita_state::app_state::AppState;

pub fn init() {
    AppState::builder()
        .with::<ThemeMode>()
        .with::<SidenavMode>()
        .with::<HeaderMode>()
        .build();
}
