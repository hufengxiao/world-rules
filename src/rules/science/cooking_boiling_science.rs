//! 煮沸与冷却科学
//!
//! 水的沸点、食物煮熟与冷却保存的科学常识

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CookingBoilingScienceRules,
    name: "煮沸与冷却科学",
    desc: "水的沸点、食物煮熟与冷却保存的科学常识",
    origin: "国际",
    tags: ["科学", "煮沸", "冷却", "烹饪"]
}

impl CookingBoilingScienceRules {
    /// 沸点常识
    pub fn boiling(&self) -> Vec<&'static str> {
        vec![
            "标准气压下沸点为百摄氏度",
            "高海拔沸点降低",
            "加盖可更快沸腾",
            "持续沸腾温度稳定",
        ]
    }

    /// 煮熟要领
    pub fn cook(&self) -> Vec<&'static str> {
        vec![
            "肉类中心充分加热",
            "蛋类煮制彻底",
            "根茎煮软入味",
            "一沸腾未必熟",
        ]
    }

    /// 冷却保存
    pub fn cool(&self) -> Vec<&'static str> {
        vec![
            "剩菜先放凉再冷藏",
            "不长时间温热存放",
            "密封防交叉污染",
            "复热彻底再食用",
        ]
    }

    /// 科学安全
    pub fn safety(&self) -> Vec<&'static str> {
        vec![
            "防止烫伤防溢锅",
            "恰当控温不闷煮",
            "注意食材配比",
            "科学烹饪保健康",
        ]
    }
}

impl Rule for CookingBoilingScienceRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("boiling_cooling")
    }

    fn explain(&self) -> String {
        format!(
            "【煮沸与冷却科学】\n{}",
            [
                format!(
                    "沸点常识：\\n{}",
                    self.boiling()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "煮熟要领：\\n{}",
                    self.cook()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "冷却保存：\\n{}",
                    self.cool()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "科学安全：\\n{}",
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
    fn test_cookingboilingsciencerules_basic() {
        let rules = CookingBoilingScienceRules::new();
        assert_eq!(rules.metadata().name, "煮沸与冷却科学");
        assert!(!rules.boiling().is_empty());
        assert!(!rules.cook().is_empty());
        assert!(!rules.cool().is_empty());
        assert!(!rules.safety().is_empty());
    }

    #[test]
    fn test_cookingboilingsciencerules_validation() {
        let rules = CookingBoilingScienceRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("boiling_cooling"));
    }

    #[test]
    fn test_cookingboilingsciencerules_explain() {
        let rules = CookingBoilingScienceRules::new();
        let e = rules.explain();
        assert!(e.contains("沸点常识"));
        assert!(e.contains("煮熟要领"));
        assert!(e.contains("冷却保存"));
    }
}
