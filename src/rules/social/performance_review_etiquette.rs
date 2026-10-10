//! 绩效面谈礼仪
//!
//! 上下级绩效评估面谈时的表达与沟通分寸

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PerformanceReviewEtiquetteRules,
    name: "绩效面谈礼仪",
    desc: "上下级绩效评估面谈时的表达与沟通分寸",
    origin: "职场通用",
    tags: ["职场", "沟通", "面谈"]
}

impl PerformanceReviewEtiquetteRules {
    /// 面谈准备
    pub fn preparation(&self) -> Vec<&'static str> {
        vec![
            "提前整理本期工作成果",
            "梳理数据与实例佐证",
            "预想可能的提问",
            "选安静不受打扰的场合",
        ]
    }

    /// 表达方式
    pub fn expression(&self) -> Vec<&'static str> {
        vec![
            "就事论事不对人",
            "用事实与数据说话",
            "先肯定再提改进",
            "避免情绪化辩解",
        ]
    }

    /// 倾听回应
    pub fn listening(&self) -> Vec<&'static str> {
        vec![
            "认真听取评价不打断",
            "有疑问礼貌提出澄清",
            "接受批评先致谢",
            "不同意见冷静陈述",
        ]
    }

    /// 争取发展
    pub fn development(&self) -> Vec<&'static str> {
        vec![
            "主动询问改进方向",
            "提出合理的资源诉求",
            "表达成长意愿与目标",
            "面谈后邮件确认共识",
        ]
    }
}

impl Rule for PerformanceReviewEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("performance_review_etiquette")
    }

    fn explain(&self) -> String {
        format!(
            "【绩效面谈礼仪】\n{}",
            [
                format!(
                    "面谈准备：\\n{}",
                    self.preparation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "表达方式：\\n{}",
                    self.expression()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "倾听回应：\\n{}",
                    self.listening()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "争取发展：\\n{}",
                    self.development()
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
    fn test_performancereviewetiquetterules_basic() {
        let rules = PerformanceReviewEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "绩效面谈礼仪");
        assert!(!rules.preparation().is_empty());
        assert!(!rules.expression().is_empty());
        assert!(!rules.listening().is_empty());
        assert!(!rules.development().is_empty());
    }

    #[test]
    fn test_performancereviewetiquetterules_validation() {
        let rules = PerformanceReviewEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::social("performance_review_etiquette")
        );
    }

    #[test]
    fn test_performancereviewetiquetterules_explain() {
        let rules = PerformanceReviewEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("面谈准备"));
        assert!(e.contains("表达方式"));
        assert!(e.contains("倾听回应"));
    }
}
