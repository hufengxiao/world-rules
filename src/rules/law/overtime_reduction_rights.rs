//! 加班与休息权益
//!
//! 加班安排、控制时间与休息休假的权利

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: OvertimeReductionRightsRules,
    name: "加班与休息权益",
    desc: "加班安排、控制时间与休息休假的权利",
    origin: "中国",
    tags: ["劳动", "加班", "休息", "权益"]
}

impl OvertimeReductionRightsRules {
    /// 工作时间
    pub fn hours(&self) -> Vec<&'static str> {
        vec![
            "标准工作时间八小时",
            "每周不超四十小时",
            "加班需依法协商",
            "保持合理作息",
        ]
    }

    /// 加班限制
    pub fn limit(&self) -> Vec<&'static str> {
        vec![
            "加班时长有限制",
            "不得强制加班",
            "特殊岗位从规",
            "休息日依法补休",
        ]
    }

    /// 加班报酬
    pub fn overtime_pay(&self) -> Vec<&'static str> {
        vec![
            "工作日加班有加班费",
            "休息日补休或加钱",
            "法定假日加班加倍",
            "核对工资条",
        ]
    }

    /// 维权路径
    pub fn protect(&self) -> Vec<&'static str> {
        vec![
            "保留加班记录",
            "证据包括日志",
            "协商不成可仲裁",
            "依法维护休息",
        ]
    }
}

impl Rule for OvertimeReductionRightsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("overtime_rights")
    }

    fn explain(&self) -> String {
        format!(
            "【加班与休息权益】\n{}",
            [
                format!(
                    "工作时间：\\n{}",
                    self.hours()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "加班限制：\\n{}",
                    self.limit()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "加班报酬：\\n{}",
                    self.overtime_pay()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "维权路径：\\n{}",
                    self.protect()
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
    fn test_overtimereductionrightsrules_basic() {
        let rules = OvertimeReductionRightsRules::new();
        assert_eq!(rules.metadata().name, "加班与休息权益");
        assert!(!rules.hours().is_empty());
        assert!(!rules.limit().is_empty());
        assert!(!rules.overtime_pay().is_empty());
        assert!(!rules.protect().is_empty());
    }

    #[test]
    fn test_overtimereductionrightsrules_validation() {
        let rules = OvertimeReductionRightsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("overtime_rights"));
    }

    #[test]
    fn test_overtimereductionrightsrules_explain() {
        let rules = OvertimeReductionRightsRules::new();
        let e = rules.explain();
        assert!(e.contains("工作时间"));
        assert!(e.contains("加班限制"));
        assert!(e.contains("加班报酬"));
    }
}
