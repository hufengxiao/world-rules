//! 音乐木头人
//!
//! 音乐停时定格不动的儿童游戏，动了即出局

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MusicalStatueRules,
    name: "音乐木头人",
    desc: "音乐停时定格不动的儿童游戏，动了即出局",
    origin: "欧美",
    tags: ["儿童", "音乐", "反应"]
}

impl MusicalStatueRules {
    /// 游戏准备
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "准备音乐播放设备",
            "一人负责控制音乐",
            "其余人自由舞动",
            "场地开阔安全",
        ]
    }

    /// 定格规则
    pub fn freeze(&self) -> Vec<&'static str> {
        vec![
            "音乐响时随意舞动",
            "音乐停立即定住",
            "保持姿势不可动",
            "动了即被淘汰",
        ]
    }

    /// 判定要点
    pub fn judging(&self) -> Vec<&'static str> {
        vec![
            "轻微晃动亦算违规",
            "可设监督者观察",
            "眨眼呼吸不算动",
            "判定须公平一致",
        ]
    }

    /// 胜负结束
    pub fn ending(&self) -> Vec<&'static str> {
        vec![
            "坚持到最后者获胜",
            "可多轮淘汰制",
            "锻炼平衡与控制力",
            "适合派对同乐",
        ]
    }
}

impl Rule for MusicalStatueRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("musical_statue")
    }

    fn explain(&self) -> String {
        format!(
            "【音乐木头人】\n{}",
            [
                format!(
                    "游戏准备：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "定格规则：\\n{}",
                    self.freeze()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "判定要点：\\n{}",
                    self.judging()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "胜负结束：\\n{}",
                    self.ending()
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
    fn test_musicalstatuerules_basic() {
        let rules = MusicalStatueRules::new();
        assert_eq!(rules.metadata().name, "音乐木头人");
        assert!(!rules.setup().is_empty());
        assert!(!rules.freeze().is_empty());
        assert!(!rules.judging().is_empty());
        assert!(!rules.ending().is_empty());
    }

    #[test]
    fn test_musicalstatuerules_validation() {
        let rules = MusicalStatueRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("musical_statue"));
    }

    #[test]
    fn test_musicalstatuerules_explain() {
        let rules = MusicalStatueRules::new();
        let e = rules.explain();
        assert!(e.contains("游戏准备"));
        assert!(e.contains("定格规则"));
        assert!(e.contains("判定要点"));
    }
}
