//! 肾结石预防
//!
//! 肾结石的预防与日常管理规则，包括充足饮水、饮食结构调整、适度运动与定期复查，降低结石形成与复发风险。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: KidneyStonePreventionRules,
    name: "肾结石预防",
    desc: "肾结石的预防与日常管理规则，包括充足饮水、饮食结构调整、适度运动与定期复查，降低结石形成与复发风险。",
    origin: "泌尿外科与营养学临床预防建议",
    tags: ["健康", "肾脏", "结石", "预防"]
}

impl KidneyStonePreventionRules {
    /// 饮水管理
    pub fn hydration(&self) -> Vec<&'static str> {
        vec![
            "每日饮水保证尿量充足",
            "均匀分配全天饮水量",
            "夜间适量补水",
            "以白开水为主少喝甜饮料",
        ]
    }

    /// 饮食调整
    pub fn diet(&self) -> Vec<&'static str> {
        vec![
            "控制高草酸食物如菠菜浓茶",
            "适量摄入钙而非完全忌钙",
            "限制高盐与高嘌呤食物",
            "多吃蔬果保持饮食均衡",
        ]
    }

    /// 生活运动
    pub fn lifestyle(&self) -> Vec<&'static str> {
        vec![
            "适度运动帮助排石",
            "避免久坐与憋尿",
            "控制体重预防代谢异常",
            "规律作息减少结石诱因",
        ]
    }

    /// 监测复查
    pub fn monitor(&self) -> Vec<&'static str> {
        vec![
            "留意腰痛血尿等信号",
            "结石成分分析指导饮食",
            "定期超声或影像复查",
            "遵医嘱调整预防方案",
        ]
    }
}

impl Rule for KidneyStonePreventionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("kidney_stone")
    }

    fn explain(&self) -> String {
        format!(
            "【肾结石预防】\n{}",
            [
                format!(
                    "饮水管理：\\n{}",
                    self.hydration()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "饮食调整：\\n{}",
                    self.diet()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "生活运动：\\n{}",
                    self.lifestyle()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "监测复查：\\n{}",
                    self.monitor()
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
    fn test_kidneystonepreventionrules_basic() {
        let rules = KidneyStonePreventionRules::new();
        assert_eq!(rules.metadata().name, "肾结石预防");
        assert!(!rules.hydration().is_empty());
        assert!(!rules.diet().is_empty());
        assert!(!rules.lifestyle().is_empty());
        assert!(!rules.monitor().is_empty());
    }

    #[test]
    fn test_kidneystonepreventionrules_validation() {
        let rules = KidneyStonePreventionRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("kidney_stone"));
    }

    #[test]
    fn test_kidneystonepreventionrules_explain() {
        let rules = KidneyStonePreventionRules::new();
        let e = rules.explain();
        assert!(e.contains("饮水管理"));
        assert!(e.contains("饮食调整"));
        assert!(e.contains("生活运动"));
    }
}
