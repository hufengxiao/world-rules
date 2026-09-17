//! 家宴款待
//!
//! 在家设宴款待客人的菜单安排、迎客与敬重礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: HostingDinnerPartyRules,
    name: "家宴款待",
    desc: "在家设宴款待客人的菜单安排、迎客与敬重礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "家宴", "款待", "聚会"]
}

impl HostingDinnerPartyRules {
    /// 宴前准备
    pub fn preparation(&self) -> Vec<&'static str> {
        vec![
            "提前确认客人饮食偏好",
            "备齐餐具座椅与布置",
            "预排上菜与饮品顺序",
            "预留足够备餐时间",
        ]
    }

    /// 迎客入席
    pub fn greeting(&self) -> Vec<&'static str> {
        vec![
            "门口迎候引导入座",
            "介绍席间彼此认识",
            "请贵宾长者居上座",
            "照顾晚到者再续座",
        ]
    }

    /// 席间款待
    pub fn serving(&self) -> Vec<&'static str> {
        vec![
            "适时上菜并分餐",
            "招呼客人夹菜不冷落",
            "关注客人饮水需求",
            "控制节奏照顾氛围",
        ]
    }

    /// 送客收尾
    pub fn farewell(&self) -> Vec<&'static str> {
        vec![
            "客人尽兴时得体收束",
            "送客至门口致谢",
            "次日或随后向宾客问候",
            "妥善处理剩餐剩菜",
        ]
    }
}

impl Rule for HostingDinnerPartyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("hosting_dinner")
    }

    fn explain(&self) -> String {
        format!(
            "【家宴款待】\n{}",
            [
                format!(
                    "宴前准备：\\n{}",
                    self.preparation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "迎客入席：\\n{}",
                    self.greeting()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "席间款待：\\n{}",
                    self.serving()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "送客收尾：\\n{}",
                    self.farewell()
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
    fn test_hostingdinnerpartyrules_basic() {
        let rules = HostingDinnerPartyRules::new();
        assert_eq!(rules.metadata().name, "家宴款待");
        assert!(!rules.preparation().is_empty());
        assert!(!rules.greeting().is_empty());
        assert!(!rules.serving().is_empty());
        assert!(!rules.farewell().is_empty());
    }

    #[test]
    fn test_hostingdinnerpartyrules_validation() {
        let rules = HostingDinnerPartyRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("hosting_dinner"));
    }

    #[test]
    fn test_hostingdinnerpartyrules_explain() {
        let rules = HostingDinnerPartyRules::new();
        let e = rules.explain();
        assert!(e.contains("宴前准备"));
        assert!(e.contains("迎客入席"));
        assert!(e.contains("席间款待"));
    }
}
