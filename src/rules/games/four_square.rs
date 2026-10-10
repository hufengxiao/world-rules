//! 四方球
//!
//! 四人各占一格的拍球淘汰游戏，流行于校园操场

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: FourSquareRules,
    name: "四方球",
    desc: "四人各占一格的拍球淘汰游戏，流行于校园操场",
    origin: "美国",
    tags: ["儿童", "球类", "校园"]
}

impl FourSquareRules {
    /// 场地布置
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "地面划出四格方块",
            "依等级标为一至四号",
            "一号位为最高级",
            "用弹性球拍击",
        ]
    }

    /// 发球规则
    pub fn serving(&self) -> Vec<&'static str> {
        vec![
            "由最高位者发球",
            "球须落入己方格再击出",
            "击球须用手掌或手臂",
            "球落对方格内为有效",
        ]
    }

    /// 出局判定
    pub fn out(&self) -> Vec<&'static str> {
        vec![
            "球出界或落己方格即出局",
            "接球失误出局",
            "持球或抓球出局",
            "出局者降至最低位",
        ]
    }

    /// 进阶规则
    pub fn advance(&self) -> Vec<&'static str> {
        vec![
            "获胜者升上一级",
            "最高位可制定附加规则",
            "空位由候补者补上",
            "轮流循环进行",
        ]
    }
}

impl Rule for FourSquareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("four_square")
    }

    fn explain(&self) -> String {
        format!(
            "【四方球】\n{}",
            [
                format!(
                    "场地布置：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "发球规则：\\n{}",
                    self.serving()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "出局判定：\\n{}",
                    self.out()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "进阶规则：\\n{}",
                    self.advance()
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
    fn test_foursquarerules_basic() {
        let rules = FourSquareRules::new();
        assert_eq!(rules.metadata().name, "四方球");
        assert!(!rules.setup().is_empty());
        assert!(!rules.serving().is_empty());
        assert!(!rules.out().is_empty());
        assert!(!rules.advance().is_empty());
    }

    #[test]
    fn test_foursquarerules_validation() {
        let rules = FourSquareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("four_square"));
    }

    #[test]
    fn test_foursquarerules_explain() {
        let rules = FourSquareRules::new();
        let e = rules.explain();
        assert!(e.contains("场地布置"));
        assert!(e.contains("发球规则"));
        assert!(e.contains("出局判定"));
    }
}
