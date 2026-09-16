//! 登山徒步礼仪
//!
//! 山地徒步、登山中的结伴、环保与让行礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MountainHikingEtiquetteRules,
    name: "登山徒步礼仪",
    desc: "山地徒步、登山中的结伴、环保与让行礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "登山", "徒步", "环保"]
}

impl MountainHikingEtiquetteRules {
    /// 结伴与出发
    pub fn buddy(&self) -> Vec<&'static str> {
        vec![
            "结伴而行不单独冒险",
            "出发前告知总体行程",
            "备足饮水干粮与御寒",
            "了解天气与路况",
        ]
    }

    /// 让行与沟通
    pub fn yielding(&self) -> Vec<&'static str> {
        vec![
            "上山者依体力自然节奏",
            "窄道秉持助力让行",
            "超队前提醒后队",
            "不占道停留大幅休息",
        ]
    }

    /// 环保山林
    pub fn leave_no_trace(&self) -> Vec<&'static str> {
        vec![
            "带走全部垃圾",
            "不折采高山植被",
            "不生明火乱丢烟头",
            "尊重野生动植物栖息",
        ]
    }

    /// 安全互助
    pub fn safety(&self) -> Vec<&'static str> {
        vec![
            "队伍前后彼此照应",
            "遇到疲惫队友主动帮扶",
            "讲清楚下撤与集合信号",
            "遇险及时求助不下深壑",
        ]
    }
}

impl Rule for MountainHikingEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("mountain_hiking")
    }

    fn explain(&self) -> String {
        format!(
            "【登山徒步礼仪】\n{}",
            [
                format!(
                    "结伴与出发：\\n{}",
                    self.buddy()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "让行与沟通：\\n{}",
                    self.yielding()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "环保山林：\\n{}",
                    self.leave_no_trace()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安全互助：\\n{}",
                    self.safety()
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
    fn test_mountainhikingetiquetterules_basic() {
        let rules = MountainHikingEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "登山徒步礼仪");
        assert!(!rules.buddy().is_empty());
        assert!(!rules.yielding().is_empty());
        assert!(!rules.leave_no_trace().is_empty());
        assert!(!rules.safety().is_empty());
    }

    #[test]
    fn test_mountainhikingetiquetterules_validation() {
        let rules = MountainHikingEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("mountain_hiking"));
    }

    #[test]
    fn test_mountainhikingetiquetterules_explain() {
        let rules = MountainHikingEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("结伴与出发"));
        assert!(e.contains("让行与沟通"));
        assert!(e.contains("环保山林"));
    }
}
