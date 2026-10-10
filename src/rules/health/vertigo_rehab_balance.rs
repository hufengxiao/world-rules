//! 眩晕与平衡康复
//!
//! 眩晕症状的日常管理与平衡康复规则，包括体位调整、前庭康复训练、环境安全与就医评估，减少跌倒风险。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: VertigoRehabBalanceRules,
    name: "眩晕与平衡康复",
    desc: "眩晕症状的日常管理与平衡康复规则，包括体位调整、前庭康复训练、环境安全与就医评估，减少跌倒风险。",
    origin: "神经内科与前庭康复医学建议",
    tags: ["健康", "眩晕", "平衡", "康复"]
}

impl VertigoRehabBalanceRules {
    /// 发作应对
    pub fn response(&self) -> Vec<&'static str> {
        vec![
            "发作时立即坐下或躺下",
            "闭眼静卧减少头部转动",
            "缓慢改变体位不猛然起身",
            "避免独自登高或驾车",
        ]
    }

    /// 平衡训练
    pub fn training(&self) -> Vec<&'static str> {
        vec![
            "循序渐进做前庭康复操",
            "练习单脚站立提升平衡",
            "扶稳物体再逐步放手",
            "量力而行不勉强",
        ]
    }

    /// 环境安全
    pub fn safety(&self) -> Vec<&'static str> {
        vec![
            "居家通道保持通畅",
            "浴室铺设防滑垫",
            "夜间留小灯防跌倒",
            "常用物品置于易取处",
        ]
    }

    /// 就医评估
    pub fn medical(&self) -> Vec<&'static str> {
        vec![
            "反复眩晕须专科检查",
            "伴耳鸣听力下降尽早就诊",
            "突发剧烈眩晕伴肢体无力急诊",
            "遵医嘱用药与复诊",
        ]
    }
}

impl Rule for VertigoRehabBalanceRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("vertigo")
    }

    fn explain(&self) -> String {
        format!(
            "【眩晕与平衡康复】\n{}",
            [
                format!(
                    "发作应对：\\n{}",
                    self.response()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "平衡训练：\\n{}",
                    self.training()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "环境安全：\\n{}",
                    self.safety()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医评估：\\n{}",
                    self.medical()
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
    fn test_vertigorehabbalancerules_basic() {
        let rules = VertigoRehabBalanceRules::new();
        assert_eq!(rules.metadata().name, "眩晕与平衡康复");
        assert!(!rules.response().is_empty());
        assert!(!rules.training().is_empty());
        assert!(!rules.safety().is_empty());
        assert!(!rules.medical().is_empty());
    }

    #[test]
    fn test_vertigorehabbalancerules_validation() {
        let rules = VertigoRehabBalanceRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("vertigo"));
    }

    #[test]
    fn test_vertigorehabbalancerules_explain() {
        let rules = VertigoRehabBalanceRules::new();
        let e = rules.explain();
        assert!(e.contains("发作应对"));
        assert!(e.contains("平衡训练"));
        assert!(e.contains("环境安全"));
    }
}
