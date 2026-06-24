//! WBC拳击规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BoxingWbcRules, name: "WBC拳击规则", desc: "WBC拳击规则", origin: "墨西哥", tags: ["体育", "格斗"] }
impl BoxingWbcRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "职业拳击12回合每回合3分钟",
            "回合间休息1分钟",
            "使用10盎司拳套(次中量级以上)",
            "裁判可终止比赛(RTKO)",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "10分制:赢方10分输方9分",
            "3名边裁打分取多数",
            "击倒:读秒10秒内无法继续判KO",
            "技术击倒:裁判医生或角终止比赛",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "迷你轻量级105磅",
            "轻蝇量级108磅",
            "蝇量级112磅",
            "雏量级118磅",
            "羽量级126磅",
            "轻量级135磅",
            "次中量级147磅",
            "中量级160磅",
            "轻重量级175磅",
            "重量级200磅以上",
        ]
    }
}
impl Rule for BoxingWbcRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("boxing_wbc")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "WBC拳击规则",
            &[
                ("比赛规则", &self.section_0()),
                ("得分规则", &self.section_1()),
                ("级别", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BoxingWbcRules::new();
        assert!(!r.explain().is_empty());
    }
}
