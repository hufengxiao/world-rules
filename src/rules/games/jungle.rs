//! 斗兽棋规则（Jungle / Dou Shou Qi）
//!
//! 中国传统棋类游戏，动物等级吃子规则，趣味性强。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: JungleRules,
    name: "斗兽棋规则",
    desc: "斗兽棋（Jungle/斗兽棋）中国传统动物棋类规则",
    origin: "中国",
    tags: ["游戏", "棋类", "斗兽棋", "动物", "传统"],
}

impl JungleRules {
    /// 历史背景
    pub fn section_history(&self) -> Vec<&'static str> {
        vec![
            "起源于中国，历史悠久",
            "动物等级吃子规则独特",
            "流行于民间，趣味性强",
            "适合儿童和初学者",
            "规则简单但策略丰富",
        ]
    }

    /// 棋盘设置
    pub fn section_board(&self) -> Vec<&'static str> {
        vec![
            "棋盘为7x9格",
            "棋盘中央有两条河流（2x3格）",
            "河流位于棋盘中央区域",
            "每方有3个陷阱（围绕兽穴）",
            "每方有1个兽穴（终点）",
        ]
    }

    /// 动物棋子等级
    pub fn section_pieces(&self) -> Vec<&'static str> {
        vec![
            "每方8枚动物棋子，等级从高到低：",
            "象（8级）：最强，但怕鼠",
            "狮（7级）：强攻棋子",
            "虎（6级）：可跳河",
            "豹（5级）：中等棋子",
            "狼（4级）：普通棋子",
            "狗（3级）：较弱棋子",
            "猫（2级）：较弱棋子",
            "鼠（1级）：最弱但可吃象，可下水",
        ]
    }

    /// 吃子规则
    pub fn section_capture(&self) -> Vec<&'static str> {
        vec![
            "大动物可吃小动物或同级动物",
            "特殊：鼠可吃象（唯一例外）",
            "象不能吃鼠",
            "陷阱中的棋子等级变为0",
            "敌方陷阱中的棋子任何动物都可吃",
        ]
    }

    /// 走法规则
    pub fn section_movement(&self) -> Vec<&'static str> {
        vec![
            "所有动物走一格（上下左右）",
            "狮虎可跳过河流（横向或纵向）",
            "鼠可进入河流游泳",
            "河中的鼠不能吃岸上的象",
            "河中的鼠可阻挡狮虎跳跃",
        ]
    }

    /// 河流规则
    pub fn section_river(&self) -> Vec<&'static str> {
        vec![
            "河流为特殊区域",
            "鼠可进入河流游泳",
            "河中的鼠可移动但不能吃岸上棋子",
            "狮虎可跳过河流（无阻挡时）",
            "河中有鼠时狮虎不可跳跃",
        ]
    }

    /// 陷阱规则
    pub fn section_trap(&self) -> Vec<&'static str> {
        vec![
            "每方有3个陷阱围绕兽穴",
            "陷阱对己方棋子无影响",
            "敌方棋子进入陷阱等级变为0",
            "陷阱中的棋子任何己方动物都可吃",
            "陷阱中的棋子离开后恢复等级",
        ]
    }

    /// 兽穴规则
    pub fn section_den(&self) -> Vec<&'static str> {
        vec![
            "兽穴是每方的终点",
            "己方棋子不能进入己方兽穴",
            "敌方棋子进入己方兽穴即获胜",
            "兽穴周围有3个陷阱保护",
            "兽穴是获胜的关键位置",
        ]
    }

    /// 获胜条件
    pub fn section_win(&self) -> Vec<&'static str> {
        vec![
            "进入对方兽穴即获胜",
            "吃光对方所有棋子即获胜",
            "对方无合法走棋判胜",
            "双方同意和棋",
            "循环僵局可判和",
        ]
    }

    /// 策略要点
    pub fn section_strategy(&self) -> Vec<&'static str> {
        vec![
            "保护好自己的鼠（可吃象）",
            "利用狮虎跳河快速进攻",
            "陷阱是关键防守位置",
            "兽穴防守很重要",
            "动物搭配协作策略",
        ]
    }
}

impl Rule for JungleRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("jungle")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "斗兽棋规则",
            &[
                ("历史背景", &self.section_history()),
                ("棋盘设置", &self.section_board()),
                ("动物棋子等级", &self.section_pieces()),
                ("吃子规则", &self.section_capture()),
                ("走法规则", &self.section_movement()),
                ("河流规则", &self.section_river()),
                ("陷阱规则", &self.section_trap()),
                ("兽穴规则", &self.section_den()),
                ("获胜条件", &self.section_win()),
                ("策略要点", &self.section_strategy()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jungle_rules_basic() {
        let rules = JungleRules::new();
        assert_eq!(rules.metadata().name, "斗兽棋规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn jungle_has_board_info() {
        let rules = JungleRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("7x9"));
        assert!(explanation.contains("河"));
    }

    #[test]
    fn jungle_has_animals() {
        let rules = JungleRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("象"));
        assert!(explanation.contains("狮"));
        assert!(explanation.contains("鼠"));
    }

    #[test]
    fn jungle_unique_rules() {
        let rules = JungleRules::new();
        let explanation = rules.explain();
        // 检查斗兽棋独特规则（鼠吃象）
        assert!(explanation.contains("鼠可吃象"));
    }

    #[test]
    fn jungle_trap_rules() {
        let rules = JungleRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("陷阱"));
        assert!(explanation.contains("兽穴"));
    }
}