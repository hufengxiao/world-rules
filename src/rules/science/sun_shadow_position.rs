//! 影子与太阳位置
//!
//! 一天中太阳位置变化使影子长短方向随之改变

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SunShadowPositionRules,
    name: "影子与太阳位置",
    desc: "一天中太阳位置变化使影子长短方向随之改变",
    origin: "中国",
    tags: ["科学", "日影", "天文", "自然"]
}

impl SunShadowPositionRules {
    /// 影子方向
    pub fn direction(&self) -> Vec<&'static str> {
        vec![
            "日出东影子向西",
            "正午影子朝北或朝南",
            "日落西影子向东",
            "随太阳移动转向",
        ]
    }

    /// 影子长短
    pub fn length(&self) -> Vec<&'static str> {
        vec![
            "早晚影子最长",
            "正午影子最短",
            "太阳越低影越长",
            "垂直照射影最短",
        ]
    }

    /// 日晷测时
    pub fn sundial(&self) -> Vec<&'static str> {
        vec![
            "立竿见影计时",
            "影子指示时刻",
            "针指时盘刻度",
            "简单天文计时",
        ]
    }

    /// 应用观察
    pub fn apply(&self) -> Vec<&'static str> {
        vec![
            "正午影子定方位",
            "影子知时刻",
            "季节不同影长短异",
            "北回归线有差异",
        ]
    }
}

impl Rule for SunShadowPositionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("sun_shadow")
    }

    fn explain(&self) -> String {
        format!(
            "【影子与太阳位置】\n{}",
            [
                format!(
                    "影子方向：\\n{}",
                    self.direction()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "影子长短：\\n{}",
                    self.length()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "日晷测时：\\n{}",
                    self.sundial()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "应用观察：\\n{}",
                    self.apply()
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
    fn test_sunshadowpositionrules_basic() {
        let rules = SunShadowPositionRules::new();
        assert_eq!(rules.metadata().name, "影子与太阳位置");
        assert!(!rules.direction().is_empty());
        assert!(!rules.length().is_empty());
        assert!(!rules.sundial().is_empty());
        assert!(!rules.apply().is_empty());
    }

    #[test]
    fn test_sunshadowpositionrules_validation() {
        let rules = SunShadowPositionRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("sun_shadow"));
    }

    #[test]
    fn test_sunshadowpositionrules_explain() {
        let rules = SunShadowPositionRules::new();
        let e = rules.explain();
        assert!(e.contains("影子方向"));
        assert!(e.contains("影子长短"));
        assert!(e.contains("日晷测时"));
    }
}
