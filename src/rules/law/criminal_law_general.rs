//! 刑法总则详解
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CriminalLawGeneralRules, name: "刑法总则详解", desc: "刑法总则详解", origin: "中国", tags: ["法律", "刑法"] }
impl CriminalLawGeneralRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "犯罪客体:犯罪行为侵害的社会关系",
            "犯罪客观方面:危害行为/危害结果/因果关系",
            "犯罪主体:实施犯罪的人(自然人/单位)",
            "犯罪主观方面:故意或过失",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "主刑:管制(3月-2年)/拘役(1月-6月)/有期徒刑(6月-15年)/无期徒刑/死刑",
            "附加刑:罚金/剥夺政治权利/没收财产/驱逐出境",
            "附加刑可独立适用也可附加适用",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "正当防卫:为保护合法权益制止不法侵害",
            "防卫过当:明显超过必要限度造成重大损害",
            "特殊防卫:对正在进行行凶杀人抢劫强奸绑架等暴力犯罪的防卫不存在防卫过当",
            "紧急避险:为保护合法权益不得已损害另一较小利益",
        ]
    }
}
impl Rule for CriminalLawGeneralRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("criminal_law_general")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "刑法总则详解",
            &[
                ("犯罪构成", &self.section_0()),
                ("刑罚种类", &self.section_1()),
                ("正当防卫", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CriminalLawGeneralRules::new();
        assert!(!r.explain().is_empty());
    }
}
