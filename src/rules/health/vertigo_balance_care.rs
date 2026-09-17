//! 眩晕与平衡调理
//!
//! 头晕眩晕的成因识别、应对与安全注意

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: VertigoBalanceCareRules,
    name: "眩晕与平衡调理",
    desc: "头晕眩晕的成因识别、应对与安全注意",
    origin: "医学",
    tags: ["健康", "眩晕", "头晕", "平衡"]
}

impl VertigoBalanceCareRules {
    /// 识别表现
    pub fn symptoms(&self) -> Vec<&'static str> {
        vec![
            "分清旋转性眩晕与头晕",
            "关注伴随恶心耳鸣",
            "注意是否与体位相关",
            "评估有无耳部感染迹象",
        ]
    }

    /// 发作应对
    pub fn response(&self) -> Vec<&'static str> {
        vec![
            "发作时立即坐下扶稳",
            "保持固定姿势减少移动",
            "闭目休息缓解眩晕",
            "避免猛然起身转头",
        ]
    }

    /// 安全防护
    pub fn safety(&self) -> Vec<&'static str> {
        vec![
            "危险活动时留意先兆",
            "不驾驶或操作精密器械",
            "居家防跌倒防磕碰",
            "身边放扶手便于扶",
        ]
    }

    /// 何时就医
    pub fn seek_care(&self) -> Vec<&'static str> {
        vec![
            "持续或反复眩晕就医",
            "伴剧烈头痛或言语不灵急诊",
            "单侧听力下降需评估",
            "通过听力与神经科检查",
        ]
    }
}

impl Rule for VertigoBalanceCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("vertigo_balance")
    }

    fn explain(&self) -> String {
        format!(
            "【眩晕与平衡调理】\n{}",
            [
                format!(
                    "识别表现：\\n{}",
                    self.symptoms()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "发作应对：\\n{}",
                    self.response()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安全防护：\\n{}",
                    self.safety()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "何时就医：\\n{}",
                    self.seek_care()
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
    fn test_vertigobalancecarerules_basic() {
        let rules = VertigoBalanceCareRules::new();
        assert_eq!(rules.metadata().name, "眩晕与平衡调理");
        assert!(!rules.symptoms().is_empty());
        assert!(!rules.response().is_empty());
        assert!(!rules.safety().is_empty());
        assert!(!rules.seek_care().is_empty());
    }

    #[test]
    fn test_vertigobalancecarerules_validation() {
        let rules = VertigoBalanceCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("vertigo_balance"));
    }

    #[test]
    fn test_vertigobalancecarerules_explain() {
        let rules = VertigoBalanceCareRules::new();
        let e = rules.explain();
        assert!(e.contains("识别表现"));
        assert!(e.contains("发作应对"));
        assert!(e.contains("安全防护"));
    }
}
