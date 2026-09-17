//! 牙缝清理与牙线
//!
//! 用牙线清理牙缝、齿间护理的方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: InterdentalFlossTipsRules,
    name: "牙缝清理与牙线",
    desc: "用牙线清理牙缝、齿间护理的方法",
    origin: "牙科",
    tags: ["健康", "牙线", "牙缝", "口腔"]
}

impl InterdentalFlossTipsRules {
    /// 正确牙线
    pub fn floss(&self) -> Vec<&'static str> {
        vec!["取一段牙线", "缠绕指上", "轻缓滑入牙缝", "上下刮面"]
    }

    /// 空心清洁
    pub fn clean(&self) -> Vec<&'static str> {
        vec!["不自碎不过用力", "逐缝清洁", "不忽视后牙", "用后清水漱"]
    }

    /// 牙龈温和
    pub fn gently(&self) -> Vec<&'static str> {
        vec!["轻柔不扎伤", "牙龈出血正常初", "坚持会好转", "出血持续就医"]
    }

    /// 辅助工具
    pub fn tools(&self) -> Vec<&'static str> {
        vec!["牙缝刷可用", "冲牙器辅助", "配合刷牙", "全面清洁"]
    }
}

impl Rule for InterdentalFlossTipsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("floss")
    }

    fn explain(&self) -> String {
        format!(
            "【牙缝清理与牙线】\n{}",
            [
                format!(
                    "正确牙线：\\n{}",
                    self.floss()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "空心清洁：\\n{}",
                    self.clean()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "牙龈温和：\\n{}",
                    self.gently()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "辅助工具：\\n{}",
                    self.tools()
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
    fn test_interdentalflosstipsrules_basic() {
        let rules = InterdentalFlossTipsRules::new();
        assert_eq!(rules.metadata().name, "牙缝清理与牙线");
        assert!(!rules.floss().is_empty());
        assert!(!rules.clean().is_empty());
        assert!(!rules.gently().is_empty());
        assert!(!rules.tools().is_empty());
    }

    #[test]
    fn test_interdentalflosstipsrules_validation() {
        let rules = InterdentalFlossTipsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("floss"));
    }

    #[test]
    fn test_interdentalflosstipsrules_explain() {
        let rules = InterdentalFlossTipsRules::new();
        let e = rules.explain();
        assert!(e.contains("正确牙线"));
        assert!(e.contains("空心清洁"));
        assert!(e.contains("牙龈温和"));
    }
}
