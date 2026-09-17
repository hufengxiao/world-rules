//! 极限飞盘规则
//!
//! 极限飞盘的基本规则、自裁精神与团队配合

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: FrisbeeUltimateRules,
    name: "极限飞盘规则",
    desc: "极限飞盘的基本规则、自裁精神与团队配合",
    origin: "国际",
    tags: ["体育", "飞盘", "极限飞盘", "团队", "自裁"]
}

impl FrisbeeUltimateRules {
    /// 基本规则
    pub fn basics(&self) -> Vec<&'static str> {
        vec![
            "通过传递飞盘推进得分",
            "持盘者在原地停顿",
            "传球越过防守者得分",
            "飞盘落地权属合理判夺",
        ]
    }

    /// 自裁精神
    pub fn spirit(&self) -> Vec<&'static str> {
        vec![
            "由选手自行判读犯规",
            "争执友好协商解决",
            "坚持公平与尊重",
            "自评与承认失误",
        ]
    }

    /// 团队配合
    pub fn teamwork(&self) -> Vec<&'static str> {
        vec![
            "无肢体接触",
            "跑动配合拉开空档",
            "传递与切入协调",
            "轮流贡献团队",
        ]
    }

    /// 安全注意
    pub fn safety(&self) -> Vec<&'static str> {
        vec![
            "留意场地与周边",
            "避免危险冲撞",
            "跑动时刻注意脚下",
            "出现伤痛立即停",
        ]
    }
}

impl Rule for FrisbeeUltimateRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("ultimate_frisbee")
    }

    fn explain(&self) -> String {
        format!(
            "【极限飞盘规则】\n{}",
            [
                format!(
                    "基本规则：\\n{}",
                    self.basics()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "自裁精神：\\n{}",
                    self.spirit()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "团队配合：\\n{}",
                    self.teamwork()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安全注意：\\n{}",
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
    fn test_frisbeeultimaterules_basic() {
        let rules = FrisbeeUltimateRules::new();
        assert_eq!(rules.metadata().name, "极限飞盘规则");
        assert!(!rules.basics().is_empty());
        assert!(!rules.spirit().is_empty());
        assert!(!rules.teamwork().is_empty());
        assert!(!rules.safety().is_empty());
    }

    #[test]
    fn test_frisbeeultimaterules_validation() {
        let rules = FrisbeeUltimateRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::sports("ultimate_frisbee"));
    }

    #[test]
    fn test_frisbeeultimaterules_explain() {
        let rules = FrisbeeUltimateRules::new();
        let e = rules.explain();
        assert!(e.contains("基本规则"));
        assert!(e.contains("自裁精神"));
        assert!(e.contains("团队配合"));
    }
}
