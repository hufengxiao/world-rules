//! 离职程序与交接
//!
//! 劳动者主动离职的程序、通知与交接要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ResignationProcedureRules,
    name: "离职程序与交接",
    desc: "劳动者主动离职的程序、通知与交接要点",
    origin: "中国",
    tags: ["法律", "离职", "辞职", "交接"]
}

impl ResignationProcedureRules {
    /// 提前通知
    pub fn notice(&self) -> Vec<&'static str> {
        vec![
            "劳动者依法提前通知",
            "试用期提前适当天",
            "正式期提前三十日书面",
            "办理好离职手续",
        ]
    }

    /// 工作交接
    pub fn handover(&self) -> Vec<&'static str> {
        vec![
            "完成工作交接",
            "归还单位财物证件",
            "做好资料移交",
            "配合单位依规办理",
        ]
    }

    /// 结清待遇
    pub fn settlement(&self) -> Vec<&'static str> {
        vec![
            "依法结清工资",
            "办理社保关系转移",
            "领取离职证明",
            "核对应得款项",
        ]
    }

    /// 离职善后
    pub fn follow(&self) -> Vec<&'static str> {
        vec![
            "遵守保密约定",
            "竞业限制依法履行",
            "保留离职证明凭证",
            "友好告别维护关系",
        ]
    }
}

impl Rule for ResignationProcedureRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("resignation")
    }

    fn explain(&self) -> String {
        format!(
            "【离职程序与交接】\n{}",
            [
                format!(
                    "提前通知：\\n{}",
                    self.notice()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "工作交接：\\n{}",
                    self.handover()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "结清待遇：\\n{}",
                    self.settlement()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "离职善后：\\n{}",
                    self.follow()
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
    fn test_resignationprocedurerules_basic() {
        let rules = ResignationProcedureRules::new();
        assert_eq!(rules.metadata().name, "离职程序与交接");
        assert!(!rules.notice().is_empty());
        assert!(!rules.handover().is_empty());
        assert!(!rules.settlement().is_empty());
        assert!(!rules.follow().is_empty());
    }

    #[test]
    fn test_resignationprocedurerules_validation() {
        let rules = ResignationProcedureRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("resignation"));
    }

    #[test]
    fn test_resignationprocedurerules_explain() {
        let rules = ResignationProcedureRules::new();
        let e = rules.explain();
        assert!(e.contains("提前通知"));
        assert!(e.contains("工作交接"));
        assert!(e.contains("结清待遇"));
    }
}
