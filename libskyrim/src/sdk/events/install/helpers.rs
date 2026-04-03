use super::batch::EventBatch;
use super::types::{EventBatchInstallResult, EventInstaller};

pub fn try_install_all(
    batch: &mut EventBatch<'_>,
    installers: &[EventInstaller],
) -> EventBatchInstallResult {
    for installer in installers {
        installer.try_install(batch)?;
    }

    Ok(())
}

pub fn install_batch_or_fatal(batch: &mut EventBatch<'_>, installers: &[EventInstaller]) {
    if let Err(error) = try_install_all(batch, installers) {
        error.install_or_fatal();
    }
}
