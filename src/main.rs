// SPDX-FileCopyrightText: 2026 Foundation Devices, Inc. <hello@foundation.xyz>
// SPDX-FileCopyrightText: 2026 BitZk contributors
// SPDX-License-Identifier: MIT

mod protocol;
mod theme;

use slint_keyos_platform::app_ui2;

app_ui2!("Vault Signer");

fn app_main(_cx: AppContext, ui: AppWindow) {
    log_server::init_wait(env!("CARGO_CRATE_NAME")).unwrap();
    log::set_max_level(log::LevelFilter::Info);

    theme::init(&ui);

    // The prototype deliberately has no seed-access callback. Keep the
    // protocol version reachable so incompatible UX and API work is visible.
    log::info!("Vault Signer UX prototype protocol v{}", protocol::PROTOCOL_VERSION);

    ui.run().expect("UI running");
}
