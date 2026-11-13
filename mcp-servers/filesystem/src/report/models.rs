use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GenerateReportArgs {
    /// Unique investigation case ID.
    pub case_id: String,

    /// Report title.
    pub title: String,

    /// Current investigation status.
    pub status: String,

    /// Executive summary of the investigation.
    pub summary: String,

    /// Key findings. Use separate lines for multiple findings.
    pub findings: String,

    /// Recommendations. Use separate lines for multiple recommendations.
    pub recommendations: String,
}
