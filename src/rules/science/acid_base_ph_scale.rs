//! 酸碱指示pH
//!
//! pH值表示酸碱、测试与生活应用

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: AcidBasePhScaleRules,
    name: "酸碱指示pH",
    desc: "pH值表示酸碱、测试与生活应用",
    origin: "化学",
    tags: ["科学", "酸碱", "pH", "化学"]
}

impl AcidBasePhScaleRules {
    /// pH含义
    pub fn ph(&self) -> Vec<&'static str> {
        vec!["pH衡量酸碱性强弱", "七为中性", "小于七偏酸", "大于七偏碱"]
    }

    /// 指示剂
    pub fn indicator(&self) -> Vec<&'static str> {
        vec!["石蕊遇酸红", "酚酞遇碱红", "试纸变色判定", "范围对照"]
    }

    /// 生活酸碱
    pub fn life(&self) -> Vec<&'static str> {
        vec!["柠檬醋属酸", "肥皂水偏碱", "平常饮食多样", "自然协调"]
    }

    /// 安全使用
    pub fn safety(&self) -> Vec<&'static str> {
        vec!["强酸碱勿沾手", "稀释沿壁倒", "实验戴防护", "谨慎操作"]
    }
}

impl Rule for AcidBasePhScaleRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("acid_base")
    }

    fn explain(&self) -> String {
        format!(
            "【酸碱指示pH】\n{}",
            [
                format!(
                    "pH含义：\\n{}",
                    self.ph()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "指示剂：\\n{}",
                    self.indicator()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "生活酸碱：\\n{}",
                    self.life()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安全使用：\\n{}",
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
    fn test_acidbasephscalerules_basic() {
        let rules = AcidBasePhScaleRules::new();
        assert_eq!(rules.metadata().name, "酸碱指示pH");
        assert!(!rules.ph().is_empty());
        assert!(!rules.indicator().is_empty());
        assert!(!rules.life().is_empty());
        assert!(!rules.safety().is_empty());
    }

    #[test]
    fn test_acidbasephscalerules_validation() {
        let rules = AcidBasePhScaleRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("acid_base"));
    }

    #[test]
    fn test_acidbasephscalerules_explain() {
        let rules = AcidBasePhScaleRules::new();
        let e = rules.explain();
        assert!(e.contains("pH含义"));
        assert!(e.contains("指示剂"));
        assert!(e.contains("生活酸碱"));
    }
}
