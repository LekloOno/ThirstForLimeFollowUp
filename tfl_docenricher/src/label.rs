use crate::error::{Error, Result};

pub fn roadmap_status_label(status: &str) -> Result<&'static str> {
    Ok(match status {
        "planned" => "\u{26AB} PLANNED",
        "cancelled" => "\u{1F534} CANCELLED",
        "delayed" => "\u{1F7E0} DELAYED",
        "wip" => "\u{1F7E1} WIP",
        "completed" => "\u{1F535} COMPLETED",
        "live" => "\u{1F7E2} LIVE",
        other => {
            return Err(Error::Generator(format!(
                "unknown roadmap_status '{other}' (expected one of planned, cancelled, delayed, wip, completed, live)"
            )))
        }
    })
}