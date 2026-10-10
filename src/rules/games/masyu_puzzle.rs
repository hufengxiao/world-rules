//! 数回
//!
//! 连接黑白圆点形成单一回路的逻辑谜题

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MasyuPuzzleRules,
    name: "数回",
    desc: "连接黑白圆点形成单一回路的逻辑谜题",
    origin: "日本",
    tags: ["解谜", "逻辑", "回路"]
}

impl MasyuPuzzleRules {
    /// 谜题布置
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "网格内含黑白圆点",
            "白圈黑圈分布各处",
            "需画线穿过所有圈",
            "形成一条闭合回路",
        ]
    }

    /// 白圈规则
    pub fn white(&self) -> Vec<&'static str> {
        vec![
            "白圈处须直线穿过",
            "圈前后至少一格后转弯",
            "不可紧邻转弯",
            "保证直线延伸",
        ]
    }

    /// 黑圈规则
    pub fn black(&self) -> Vec<&'static str> {
        vec![
            "黑圈处须转弯",
            "转弯前后须直行一格",
            "不可紧邻再转弯",
            "两段直线延伸",
        ]
    }

    /// 回路规则
    pub fn loop_path(&self) -> Vec<&'static str> {
        vec![
            "回路不可交叉或分叉",
            "须经过所有圆点",
            "其余格可不经过",
            "最终闭合成单环",
        ]
    }
}

impl Rule for MasyuPuzzleRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("masyu_puzzle")
    }

    fn explain(&self) -> String {
        format!(
            "【数回】\n{}",
            [
                format!(
                    "谜题布置：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "白圈规则：\\n{}",
                    self.white()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "黑圈规则：\\n{}",
                    self.black()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "回路规则：\\n{}",
                    self.loop_path()
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
    fn test_masyupuzzlerules_basic() {
        let rules = MasyuPuzzleRules::new();
        assert_eq!(rules.metadata().name, "数回");
        assert!(!rules.setup().is_empty());
        assert!(!rules.white().is_empty());
        assert!(!rules.black().is_empty());
        assert!(!rules.loop_path().is_empty());
    }

    #[test]
    fn test_masyupuzzlerules_validation() {
        let rules = MasyuPuzzleRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("masyu_puzzle"));
    }

    #[test]
    fn test_masyupuzzlerules_explain() {
        let rules = MasyuPuzzleRules::new();
        let e = rules.explain();
        assert!(e.contains("谜题布置"));
        assert!(e.contains("白圈规则"));
        assert!(e.contains("黑圈规则"));
    }
}
