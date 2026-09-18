//! 委婉拒绝
//!
//! 不好意思直接答应时礼貌又不失体面的拒绝方式

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PolitelyRefusingRules,
    name: "委婉拒绝",
    desc: "不好意思直接答应时礼貌又不失体面的拒绝方式",
    origin: "中国",
    tags: ["社交", "拒绝", "礼貌", "边界"]
}

impl PolitelyRefusingRules {
    /// 拒绝态度
    pub fn attitude(&self) -> Vec<&'static str> {
        vec!["态度温和坚定", "先谢对方好意", "直说不绕圈子", "不模棱两可"]
    }

    /// 委婉说法
    pub fn wording(&self) -> Vec<&'static str> {
        vec![
            "这次恐怕不行",
            "时间上安排不开",
            "请理解我的难处",
            "换个方式帮帮你",
        ]
    }

    /// 给个台阶
    pub fn soft(&self) -> Vec<&'static str> {
        vec![
            "解释但不过度",
            "不扯谎找借口",
            "可提替代方案",
            "缓和关系不生硬",
        ]
    }

    /// 守住边界
    pub fn boundary(&self) -> Vec<&'static str> {
        vec![
            "对不合理要求说不",
            "不委曲全答应",
            "说开打消误会",
            "保持互相尊重",
        ]
    }
}

impl Rule for PolitelyRefusingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("politely_refuse")
    }

    fn explain(&self) -> String {
        format!(
            "【委婉拒绝】\n{}",
            [
                format!(
                    "拒绝态度：\\n{}",
                    self.attitude()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "委婉说法：\\n{}",
                    self.wording()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "给个台阶：\\n{}",
                    self.soft()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "守住边界：\\n{}",
                    self.boundary()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
            ]
            .join("\n\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_politelyrefusingrules_basic() {
        let rules = PolitelyRefusingRules::new();
        assert_eq!(rules.metadata().name, "委婉拒绝");
        assert!(!rules.attitude().is_empty());
        assert!(!rules.wording().is_empty());
        assert!(!rules.soft().is_empty());
        assert!(!rules.boundary().is_empty());
    }

    #[test]
    fn test_politelyrefusingrules_validation() {
        let rules = PolitelyRefusingRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("politely_refuse"));
    }

    #[test]
    fn test_politelyrefusingrules_explain() {
        let rules = PolitelyRefusingRules::new();
        let e = rules.explain();
        assert!(e.contains("拒绝态度"));
        assert!(e.contains("委婉说法"));
        assert!(e.contains("给个台阶"));
    }
}
