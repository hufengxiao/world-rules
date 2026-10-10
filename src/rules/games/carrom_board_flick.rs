//! 康乐棋（Carrom）
//!
//! 康乐棋是一种在方形木盘上用指弹击打棋子入袋的桌面游戏，玩家轮流用手指弹击己方棋子，将棋子击入四角球袋得分。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CarromBoardFlickRules,
    name: "康乐棋（Carrom）",
    desc: "康乐棋是一种在方形木盘上用指弹击打棋子入袋的桌面游戏，玩家轮流用手指弹击己方棋子，将棋子击入四角球袋得分。",
    origin: "起源于印度次大陆，流行于南亚与东南亚",
    tags: ["桌面游戏", "弹击", "竞技", "技巧"]
}

impl CarromBoardFlickRules {
    /// 器材与布局
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "方形木盘四角设球袋",
            "棋子分黑方与白方及一枚红王",
            "摆成花朵状居于盘心",
            "使用专用滑粉减少摩擦",
        ]
    }

    /// 击打规则
    pub fn striking(&self) -> Vec<&'static str> {
        vec![
            "用手指弹击己方棋子",
            "击打时手不得越线进入盘面",
            "击打后棋子须有移动或触及其他棋子",
            "不得直接击打对方棋子",
        ]
    }

    /// 得分与犯规
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "击入己方棋子得一分",
            "误入对方棋子判犯规",
            "己方棋子入袋后可继续击打",
            "未得分则轮换对手",
        ]
    }

    /// 胜负判定
    pub fn winning(&self) -> Vec<&'static str> {
        vec![
            "先击入全部己方棋子者准备收王",
            "击入红王须有己方棋子已入袋",
            "最后合法击入红王者获胜",
            "违规入王须归还并罚棋",
        ]
    }
}

impl Rule for CarromBoardFlickRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("carrom")
    }

    fn explain(&self) -> String {
        format!(
            "【康乐棋（Carrom）】\n{}",
            [
                format!(
                    "器材与布局：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "击打规则：\\n{}",
                    self.striking()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "得分与犯规：\\n{}",
                    self.scoring()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "胜负判定：\\n{}",
                    self.winning()
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
    fn test_carromboardflickrules_basic() {
        let rules = CarromBoardFlickRules::new();
        assert_eq!(rules.metadata().name, "康乐棋（Carrom）");
        assert!(!rules.setup().is_empty());
        assert!(!rules.striking().is_empty());
        assert!(!rules.scoring().is_empty());
        assert!(!rules.winning().is_empty());
    }

    #[test]
    fn test_carromboardflickrules_validation() {
        let rules = CarromBoardFlickRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("carrom"));
    }

    #[test]
    fn test_carromboardflickrules_explain() {
        let rules = CarromBoardFlickRules::new();
        let e = rules.explain();
        assert!(e.contains("器材与布局"));
        assert!(e.contains("击打规则"));
        assert!(e.contains("得分与犯规"));
    }
}
