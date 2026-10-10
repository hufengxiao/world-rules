//! 婴儿车出行礼仪
//!
//! 推婴儿车在公共场所出行的礼仪规则，包括通行礼让、公共交通乘坐、电梯使用与安全注意，兼顾育儿便利与他人感受。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BabyStrollerMannerRules,
    name: "婴儿车出行礼仪",
    desc: "推婴儿车在公共场所出行的礼仪规则，包括通行礼让、公共交通乘坐、电梯使用与安全注意，兼顾育儿便利与他人感受。",
    origin: "城市公共空间中育儿出行场景",
    tags: ["社交", "公共", "育儿", "礼仪"]
}

impl BabyStrollerMannerRules {
    /// 通行礼让
    pub fn passing(&self) -> Vec<&'static str> {
        vec![
            "狭窄通道主动礼让行人",
            "不占据通道长时间停留",
            "进出门口让他人先行",
            "人多处减速慢行",
        ]
    }

    /// 公共交通
    pub fn transit(&self) -> Vec<&'static str> {
        vec![
            "上下车先收折或避让",
            "不妨碍车门与通道",
            "乘坐扶梯优先选直梯",
            "遇拥挤耐心等候下一班",
        ]
    }

    /// 电梯使用
    pub fn elevator(&self) -> Vec<&'static str> {
        vec![
            "推车进电梯先打招呼",
            "尽量靠边不挡按键",
            "让出行人空间",
            "进出先确认安全",
        ]
    }

    /// 安全与体谅
    pub fn safety(&self) -> Vec<&'static str> {
        vec![
            "系好安全带防止跌落",
            "不将婴儿车停在楼梯口",
            "注意车轮不碾压他人",
            "对让路者道谢",
        ]
    }
}

impl Rule for BabyStrollerMannerRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("stroller")
    }

    fn explain(&self) -> String {
        format!(
            "【婴儿车出行礼仪】\n{}",
            [
                format!(
                    "通行礼让：\\n{}",
                    self.passing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "公共交通：\\n{}",
                    self.transit()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "电梯使用：\\n{}",
                    self.elevator()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安全与体谅：\\n{}",
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
    fn test_babystrollermannerrules_basic() {
        let rules = BabyStrollerMannerRules::new();
        assert_eq!(rules.metadata().name, "婴儿车出行礼仪");
        assert!(!rules.passing().is_empty());
        assert!(!rules.transit().is_empty());
        assert!(!rules.elevator().is_empty());
        assert!(!rules.safety().is_empty());
    }

    #[test]
    fn test_babystrollermannerrules_validation() {
        let rules = BabyStrollerMannerRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("stroller"));
    }

    #[test]
    fn test_babystrollermannerrules_explain() {
        let rules = BabyStrollerMannerRules::new();
        let e = rules.explain();
        assert!(e.contains("通行礼让"));
        assert!(e.contains("公共交通"));
        assert!(e.contains("电梯使用"));
    }
}
