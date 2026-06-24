//! 轮椅竞速IPC
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: WheelchairRacingIpcRules, name: "轮椅竞速IPC", desc: "IPC轮椅竞速规则", origin: "国际", tags: ["体育", "残疾人"] }
impl WheelchairRacingIpcRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["T51到T54"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["100m到马拉松"]
    }
}
impl Rule for WheelchairRacingIpcRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("wheelchair_racing_ipc")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "轮椅竞速IPC",
            &[("分级", &self.section_0()), ("项目", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = WheelchairRacingIpcRules::new();
        assert!(!r.explain().is_empty());
    }
}
