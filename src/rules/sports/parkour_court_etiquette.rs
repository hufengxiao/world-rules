//! 跑酷锻炼规范
//!
//! 跑酷场地训练的安全、规范与公共空间礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ParkourCourtEtiquetteRules,
    name: "跑酷锻炼规范",
    desc: "跑酷场地训练的安全、规范与公共空间礼仪",
    origin: "国际",
    tags: ["体育", "跑酷", "训练", "安全"]
}

impl ParkourCourtEtiquetteRules {
    /// 安全第一
    pub fn safety(&self) -> Vec<&'static str> {
        vec![
            "先从基础动作练起",
            "穿戴护具防受伤",
            "热身充分再练",
            "不攀危险高处",
        ]
    }

    /// 场地尊重
    pub fn court(&self) -> Vec<&'static str> {
        vec![
            "选择适宜场地",
            "不破坏公共设施",
            "避开人流车辆",
            "不占他人锻炼空间",
        ]
    }

    /// 循序渐进
    pub fn progression(&self) -> Vec<&'static str> {
        vec![
            "由易到难逐步提升",
            "有教练指导更稳妥",
            "不盲目尝试高难",
            "量力而行勿逞强",
        ]
    }

    /// 社区规范
    pub fn etiquette(&self) -> Vec<&'static str> {
        vec![
            "不惊扰他人",
            "未经许可翻越区域",
            "爱护环境不破坏",
            "传递正面形象",
        ]
    }
}

impl Rule for ParkourCourtEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("parkour")
    }

    fn explain(&self) -> String {
        format!(
            "【跑酷锻炼规范】\n{}",
            [
                format!(
                    "安全第一：\\n{}",
                    self.safety()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "场地尊重：\\n{}",
                    self.court()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "循序渐进：\\n{}",
                    self.progression()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "社区规范：\\n{}",
                    self.etiquette()
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
    fn test_parkourcourtetiquetterules_basic() {
        let rules = ParkourCourtEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "跑酷锻炼规范");
        assert!(!rules.safety().is_empty());
        assert!(!rules.court().is_empty());
        assert!(!rules.progression().is_empty());
        assert!(!rules.etiquette().is_empty());
    }

    #[test]
    fn test_parkourcourtetiquetterules_validation() {
        let rules = ParkourCourtEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::sports("parkour"));
    }

    #[test]
    fn test_parkourcourtetiquetterules_explain() {
        let rules = ParkourCourtEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("安全第一"));
        assert!(e.contains("场地尊重"));
        assert!(e.contains("循序渐进"));
    }
}
