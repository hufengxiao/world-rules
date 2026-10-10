//! Blokus 拼块棋
//!
//! Blokus 是四人拼块对弈游戏，玩家在棋盘上摆放各自颜色的多格方块，要求同色方块只能角对角相连，尽量多占棋盘并封锁对手。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BlokusPiecesRules,
    name: "Blokus 拼块棋",
    desc: "Blokus 是四人拼块对弈游戏，玩家在棋盘上摆放各自颜色的多格方块，要求同色方块只能角对角相连，尽量多占棋盘并封锁对手。",
    origin: "由 Bernard Tavitian 于2000年发明",
    tags: ["拼块", "对战", "空间", "桌面游戏"]
}

impl BlokusPiecesRules {
    /// 棋盘与方块
    pub fn board(&self) -> Vec<&'static str> {
        vec![
            "棋盘为二十乘二十方格",
            "四方各有一套二十一块方块",
            "方块形状由一到五格组成",
            "四角为各自起始点",
        ]
    }

    /// 摆放规则
    pub fn placing(&self) -> Vec<&'static str> {
        vec![
            "首块须覆盖己方起始角",
            "同色方块只能角对角相接",
            "同色方块边与边不得接触",
            "不同色方块边可相邻",
        ]
    }

    /// 回合流程
    pub fn turn(&self) -> Vec<&'static str> {
        vec![
            "按顺序轮流出块",
            "每回合须放一块或声明跳过",
            "无处可放则跳过",
            "全部无法落子则本局结束",
        ]
    }

    /// 计分判定
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "每放下一格方块得一分",
            "剩余方块每格扣一分",
            "放完所有方块额外加奖励分",
            "得分最高者获胜",
        ]
    }
}

impl Rule for BlokusPiecesRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("blokus")
    }

    fn explain(&self) -> String {
        format!(
            "【Blokus 拼块棋】\n{}",
            [
                format!(
                    "棋盘与方块：\\n{}",
                    self.board()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "摆放规则：\\n{}",
                    self.placing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "回合流程：\\n{}",
                    self.turn()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "计分判定：\\n{}",
                    self.scoring()
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
    fn test_blokuspiecesrules_basic() {
        let rules = BlokusPiecesRules::new();
        assert_eq!(rules.metadata().name, "Blokus 拼块棋");
        assert!(!rules.board().is_empty());
        assert!(!rules.placing().is_empty());
        assert!(!rules.turn().is_empty());
        assert!(!rules.scoring().is_empty());
    }

    #[test]
    fn test_blokuspiecesrules_validation() {
        let rules = BlokusPiecesRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("blokus"));
    }

    #[test]
    fn test_blokuspiecesrules_explain() {
        let rules = BlokusPiecesRules::new();
        let e = rules.explain();
        assert!(e.contains("棋盘与方块"));
        assert!(e.contains("摆放规则"));
        assert!(e.contains("回合流程"));
    }
}
