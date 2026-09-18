//! 战舰猜位
//!
//! 在坐标格上猜测并击沉对手舰船的回合对战游戏

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BattleshipBoardShipRules,
    name: "战舰猜位",
    desc: "在坐标格上猜测并击沉对手舰船的回合对战游戏",
    origin: "美国",
    tags: ["游戏", "战舰", "对战", "策略"]
}

impl BattleshipBoardShipRules {
    /// 布局规则
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "双方各自布舰",
            "九乘十字格",
            "舰船占相邻格",
            "布局保密不泄露",
        ]
    }

    /// 开火回合
    pub fn fire(&self) -> Vec<&'static str> {
        vec![
            "轮流报出坐标",
            "命中对方要落格",
            "未中可记空点",
            "你打我一打你",
        ]
    }

    /// 判定击沉
    pub fn sink(&self) -> Vec<&'static str> {
        vec!["填满整艘舰格", "即判定击沉", "通报舰型", "全部击沉对方败"]
    }

    /// 策略思路
    pub fn strategy(&self) -> Vec<&'static str> {
        vec![
            "规律分布预判",
            "命中后沿向延伸",
            "记清未试区域",
            "交替扫点提高",
        ]
    }
}

impl Rule for BattleshipBoardShipRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("battleship")
    }

    fn explain(&self) -> String {
        format!(
            "【战舰猜位】\n{}",
            [
                format!(
                    "布局规则：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "开火回合：\\n{}",
                    self.fire()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "判定击沉：\\n{}",
                    self.sink()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "策略思路：\\n{}",
                    self.strategy()
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
    fn test_battleshipboardshiprules_basic() {
        let rules = BattleshipBoardShipRules::new();
        assert_eq!(rules.metadata().name, "战舰猜位");
        assert!(!rules.setup().is_empty());
        assert!(!rules.fire().is_empty());
        assert!(!rules.sink().is_empty());
        assert!(!rules.strategy().is_empty());
    }

    #[test]
    fn test_battleshipboardshiprules_validation() {
        let rules = BattleshipBoardShipRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("battleship"));
    }

    #[test]
    fn test_battleshipboardshiprules_explain() {
        let rules = BattleshipBoardShipRules::new();
        let e = rules.explain();
        assert!(e.contains("布局规则"));
        assert!(e.contains("开火回合"));
        assert!(e.contains("判定击沉"));
    }
}
