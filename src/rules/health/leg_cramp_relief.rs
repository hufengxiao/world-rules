//! 腿部抽筋与痉挛
//!
//! 夜间腿抽筋等痉挛的缓解拉伸与预防

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: LegCrampReliefRules,
    name: "腿部抽筋与痉挛",
    desc: "夜间腿抽筋等痉挛的缓解拉伸与预防",
    origin: "医学",
    tags: ["健康", "抽筋", "小腿", "痉挛"]
}

impl LegCrampReliefRules {
    /// 急性缓解
    pub fn relief(&self) -> Vec<&'static str> {
        vec![
            "抽筋时立即伸展该侧肌肉",
            "脚趾向身体方向扳拉",
            "按摩或热敷小腿",
            "走动借助重量帮助拉伸",
        ]
    }

    /// 夜间预防
    pub fn night(&self) -> Vec<&'static str> {
        vec![
            "睡前适度拉伸小腿",
            "保持舒适睡姿",
            "被子不要过压脚尖",
            "睡前适度补水",
        ]
    }

    /// 日常调养
    pub fn lifestyle(&self) -> Vec<&'static str> {
        vec![
            "补充水分与电解质",
            "摄入含镁钾钙食物",
            "规律运动不过度疲劳",
            "穿合适鞋子站立",
        ]
    }

    /// 就医提示
    pub fn seek(&self) -> Vec<&'static str> {
        vec![
            "频繁抽筋或伴肿胀就医",
            "伴走路痛或变色检查",
            "孕妇频繁痉挛咨询",
            "伴随药物副作用评估",
        ]
    }
}

impl Rule for LegCrampReliefRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("leg_cramp")
    }

    fn explain(&self) -> String {
        format!(
            "【腿部抽筋与痉挛】\n{}",
            [
                format!(
                    "急性缓解：\\n{}",
                    self.relief()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "夜间预防：\\n{}",
                    self.night()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "日常调养：\\n{}",
                    self.lifestyle()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医提示：\\n{}",
                    self.seek()
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
    fn test_legcrampreliefrules_basic() {
        let rules = LegCrampReliefRules::new();
        assert_eq!(rules.metadata().name, "腿部抽筋与痉挛");
        assert!(!rules.relief().is_empty());
        assert!(!rules.night().is_empty());
        assert!(!rules.lifestyle().is_empty());
        assert!(!rules.seek().is_empty());
    }

    #[test]
    fn test_legcrampreliefrules_validation() {
        let rules = LegCrampReliefRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("leg_cramp"));
    }

    #[test]
    fn test_legcrampreliefrules_explain() {
        let rules = LegCrampReliefRules::new();
        let e = rules.explain();
        assert!(e.contains("急性缓解"));
        assert!(e.contains("夜间预防"));
        assert!(e.contains("日常调养"));
    }
}
