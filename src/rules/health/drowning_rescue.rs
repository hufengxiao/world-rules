//! 溺水救援安全
//!
//! 发现溺水者时安全的呼救、施救与急救要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: DrowningRescueRules,
    name: "溺水救援安全",
    desc: "发现溺水者时安全的呼救、施救与急救要点",
    origin: "安全",
    tags: ["健康", "溺水", "救援", "急救"]
}

impl DrowningRescueRules {
    /// 先确保自身安全
    pub fn own_safety(&self) -> Vec<&'static str> {
        vec![
            "不贸然贸然徒手救人",
            "先大声呼救取潜水器材",
            "远距离用竿绳等抛投救助",
            "不具备能力不盲目下水",
        ]
    }

    /// 实施救援
    pub fn rescue(&self) -> Vec<&'static str> {
        vec![
            "将患者救至安全岸边",
            "清除口鼻异物通畅气道",
            "意识不清及时呼急救并观察呼吸",
            "按需实施按压或人工呼吸",
        ]
    }

    /// 拍背排水误区
    pub fn caution(&self) -> Vec<&'static str> {
        vec![
            "不确需排腹水则专注救援",
            "勿长时间倒挂拍背延误按压",
            "患者无反应先判断呼吸脉搏",
            "保持其保暖防失温",
        ]
    }

    /// 就医诊断
    pub fn care(&self) -> Vec<&'static str> {
        vec![
            "溺水后即使恢复也建议就医",
            "警惕迟发性肺部问题",
            "呼急救不因患者咳嗽即离开",
            "岸边等待医护观察",
        ]
    }
}

impl Rule for DrowningRescueRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("drowning_rescue")
    }

    fn explain(&self) -> String {
        format!(
            "【溺水救援安全】\n{}",
            [
                format!(
                    "先确保自身安全：\\n{}",
                    self.own_safety()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "实施救援：\\n{}",
                    self.rescue()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "拍背排水误区：\\n{}",
                    self.caution()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医诊断：\\n{}",
                    self.care()
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
    fn test_drowningrescuerules_basic() {
        let rules = DrowningRescueRules::new();
        assert_eq!(rules.metadata().name, "溺水救援安全");
        assert!(!rules.own_safety().is_empty());
        assert!(!rules.rescue().is_empty());
        assert!(!rules.caution().is_empty());
        assert!(!rules.care().is_empty());
    }

    #[test]
    fn test_drowningrescuerules_validation() {
        let rules = DrowningRescueRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("drowning_rescue"));
    }

    #[test]
    fn test_drowningrescuerules_explain() {
        let rules = DrowningRescueRules::new();
        let e = rules.explain();
        assert!(e.contains("先确保自身安全"));
        assert!(e.contains("实施救援"));
        assert!(e.contains("拍背排水误区"));
    }
}
