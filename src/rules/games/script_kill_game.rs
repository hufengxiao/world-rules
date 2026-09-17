//! 剧本杀桌游规范
//!
//! 剧本杀角色扮演、推理与游戏礼仪规范

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ScriptKillGameRules,
    name: "剧本杀桌游规范",
    desc: "剧本杀角色扮演、推理与游戏礼仪规范",
    origin: "中国",
    tags: ["游戏", "剧本杀", "桌游", "推理"]
}

impl ScriptKillGameRules {
    /// 角色带入
    pub fn roleplay(&self) -> Vec<&'static str> {
        vec![
            "认真阅读剧本",
            "按角色设定演绎",
            "不剧透他人",
            "守好自己秘密",
        ]
    }

    /// 推理协作
    pub fn deduction(&self) -> Vec<&'static str> {
        vec![
            "围绕线索推理",
            "分享发现合乎逻辑",
            "不跳车偷看线索",
            "配合推进剧情",
        ]
    }

    /// 复盘交流
    pub fn reveal(&self) -> Vec<&'static str> {
        vec![
            "结局揭晓客观看待",
            "不指控针对个人",
            "复盘客观友好",
            "体验重于胜负",
        ]
    }

    /// 桌游礼仪
    pub fn manner(&self) -> Vec<&'static str> {
        vec!["守时不迟到", "不中途离场", "尊重主持人", "保持友好氛围"]
    }
}

impl Rule for ScriptKillGameRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("script_kill")
    }

    fn explain(&self) -> String {
        format!(
            "【剧本杀桌游规范】\n{}",
            [
                format!(
                    "角色带入：\\n{}",
                    self.roleplay()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "推理协作：\\n{}",
                    self.deduction()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "复盘交流：\\n{}",
                    self.reveal()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "桌游礼仪：\\n{}",
                    self.manner()
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
    fn test_scriptkillgamerules_basic() {
        let rules = ScriptKillGameRules::new();
        assert_eq!(rules.metadata().name, "剧本杀桌游规范");
        assert!(!rules.roleplay().is_empty());
        assert!(!rules.deduction().is_empty());
        assert!(!rules.reveal().is_empty());
        assert!(!rules.manner().is_empty());
    }

    #[test]
    fn test_scriptkillgamerules_validation() {
        let rules = ScriptKillGameRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("script_kill"));
    }

    #[test]
    fn test_scriptkillgamerules_explain() {
        let rules = ScriptKillGameRules::new();
        let e = rules.explain();
        assert!(e.contains("角色带入"));
        assert!(e.contains("推理协作"));
        assert!(e.contains("复盘交流"));
    }
}
