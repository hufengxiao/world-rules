//! 工作交接礼仪
//!
//! 离岗或转岗时与接手同事的交接规范与配合分寸

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: WorkHandoverEtiquetteRules,
    name: "工作交接礼仪",
    desc: "离岗或转岗时与接手同事的交接规范与配合分寸",
    origin: "职场通用",
    tags: ["职场", "交接", "协作"]
}

impl WorkHandoverEtiquetteRules {
    /// 交接准备
    pub fn preparation(&self) -> Vec<&'static str> {
        vec![
            "整理任务清单与进度",
            "标注关键节点与风险点",
            "归档文件统一存放位置",
            "列出对接人与联系方式",
        ]
    }

    /// 交接沟通
    pub fn communication(&self) -> Vec<&'static str> {
        vec![
            "约定充足时间当面交接",
            "逐项讲解不留遗漏",
            "耐心解答接手方疑问",
            "重要事项书面确认",
        ]
    }

    /// 过渡配合
    pub fn transition(&self) -> Vec<&'static str> {
        vec![
            "交接后留缓冲期答疑",
            "保持联系方式畅通",
            "不越权插手新负责人事务",
            "被问及时如实协助",
        ]
    }

    /// 责任边界
    pub fn boundary(&self) -> Vec<&'static str> {
        vec![
            "交接完成须双方签字",
            "遗留问题如实说明",
            "不隐瞒潜在隐患",
            "离岗后不推诿旧责",
        ]
    }
}

impl Rule for WorkHandoverEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("work_handover_etiquette")
    }

    fn explain(&self) -> String {
        format!(
            "【工作交接礼仪】\n{}",
            [
                format!(
                    "交接准备：\\n{}",
                    self.preparation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "交接沟通：\\n{}",
                    self.communication()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "过渡配合：\\n{}",
                    self.transition()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "责任边界：\\n{}",
                    self.boundary()
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
    fn test_workhandoveretiquetterules_basic() {
        let rules = WorkHandoverEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "工作交接礼仪");
        assert!(!rules.preparation().is_empty());
        assert!(!rules.communication().is_empty());
        assert!(!rules.transition().is_empty());
        assert!(!rules.boundary().is_empty());
    }

    #[test]
    fn test_workhandoveretiquetterules_validation() {
        let rules = WorkHandoverEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::social("work_handover_etiquette")
        );
    }

    #[test]
    fn test_workhandoveretiquetterules_explain() {
        let rules = WorkHandoverEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("交接准备"));
        assert!(e.contains("交接沟通"));
        assert!(e.contains("过渡配合"));
    }
}
