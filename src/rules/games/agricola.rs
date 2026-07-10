//! 农场乐规则（Agricola）
//!
//! 德国经典农场桌游，农场建设与家庭养育。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: AgricolaRules,
    name: "农场乐规则",
    desc: "农场乐（Agricola）农场经营桌游规则",
    origin: "德国",
    tags: ["游戏", "桌游", "农场乐", "策略", "资源管理"],
}

impl AgricolaRules {
    /// 游戏概述
    pub fn section_overview(&self) -> Vec<&'static str> {
        vec![
            "德国桌游，2007年出版",
            "设计师 Uwe Rosenberg",
            "玩家经营农场和家庭",
            "建设房屋、农田、饲养动物",
            "终局得分最高者获胜",
        ]
    }

    /// 游戏组件
    pub fn section_components(&self) -> Vec<&'static str> {
        vec![
            "玩家农场板：每人一块农场",
            "行动格：17种行动格",
            "资源：木材、砖块、石头",
            "职业牌：多种职业",
            "次要改善牌：各种设施",
        ]
    }

    /// 资源类型
    pub fn section_resources(&self) -> Vec<&'static str> {
        vec![
            "木材：基础建设材料",
            "砖块：升级房屋材料",
            "石头：高级建筑材料",
            "谷物：食物和种植",
            "蔬菜：食物和种植",
        ]
    }

    /// 动物种类
    pub fn section_animals(&self) -> Vec<&'static str> {
        vec![
            "羊：初始牧场动物",
            "牛：后期高价值动物",
            "猪：中等价值动物",
            "动物需要栅栏圈养",
            "动物繁殖提供食物和得分",
        ]
    }

    /// 游戏回合
    pub fn section_rounds(&self) -> Vec<&'static str> {
        vec![
            "共14回合",
            "每回合：收获阶段 + 行动阶段",
            "收获阶段：获得资源/繁殖动物",
            "行动阶段：选择行动格",
            "每人每回合只能选一个行动",
        ]
    }

    /// 行动格类型
    pub fn section_actions(&self) -> Vec<&'static str> {
        vec![
            "基础行动：如取木材、取砖块",
            "累积行动：资源每回合累积",
            "建设行动：建房屋、建农田",
            "动物行动：获取动物",
            "家庭行动：增加家庭成员",
        ]
    }

    /// 农场建设
    pub fn section_farm_building(&self) -> Vec<&'static str> {
        vec![
            "农田：种植谷物和蔬菜",
            "栅栏：圈养动物",
            "房屋：木屋→砖屋→石屋",
            "炉灶：烹饪食物",
            "改善设施：提供特殊效果",
        ]
    }

    /// 房屋升级
    pub fn section_housing(&self) -> Vec<&'static str> {
        vec![
            "木屋：初始2人房间",
            "砖屋：升级花费砖块",
            "石屋：升级花费石头",
            "房间数量决定家庭成员上限",
            "高级房屋得分更高",
        ]
    }

    /// 家庭成员
    pub fn section_family(&self) -> Vec<&'static str> {
        vec![
            "初始2名家庭成员",
            "增加成员需要房间",
            "每个成员提供额外行动",
            "最多5名成员",
            "终局每成员得分",
        ]
    }

    /// 食物需求
    pub fn section_food(&self) -> Vec<&'static str> {
        vec![
            "每次收获需要喂饱家庭成员",
            "每成员需要2食物",
            "食物来源：谷物、蔬菜、动物",
            "炉灶可以烹饪",
            "缺乏食物惩罚严重",
        ]
    }

    /// 收获阶段
    pub fn section_harvest(&self) -> Vec<&'static str> {
        vec![
            "回合4、7、9、11、13、14有收获",
            "农田产出：谷物和蔬菜",
            "动物繁殖：每种+1",
            "喂养家庭成员",
            "缺食物：乞讨牌惩罚(-3VP)",
        ]
    }

    /// 计分系统
    pub fn section_scoring(&self) -> Vec<&'static str> {
        vec![
            "农田：每块1分",
            "谷物/蔬菜：每种+1分，空田-1分",
            "动物：数量对应得分",
            "房屋：类型和房间数得分",
            "家庭成员：每名3分",
            "职业牌/改善牌：特殊加分",
        ]
    }

    /// 负分规则
    pub fn section_penalties(&self) -> Vec<&'static str> {
        vec![
            "空农田：-1分",
            "无谷物：-1分",
            "无蔬菜：-1分",
            "无羊：-1分",
            "无猪/牛：各-1分",
            "乞讨牌：每张-3分",
        ]
    }

    /// 游戏结束
    pub fn section_endgame(&self) -> Vec<&'static str> {
        vec![
            "14回合结束后计分",
            "计算所有得分和惩罚",
            "总分最高者获胜",
            "负分可能降至最低",
            "完美农场难得高分",
        ]
    }

    /// 策略要点
    pub fn section_strategy(&self) -> Vec<&'static str> {
        vec![
            "早期建立食物基础",
            "中期建设农田和牧场",
            "平衡房屋升级与成员增加",
            "避免负分（尤其乞讨）",
            "职业牌组合很重要",
        ]
    }

    /// 变体版本
    pub fn section_variants(&self) -> Vec<&'static str> {
        vec![
            "家庭版：简化规则",
            "完整版：职业牌和改善牌",
            "两人版：特殊行动格",
            "农场乐：所有动物：专注动物",
            "Caverna：农场乐后续版本",
        ]
    }
}

impl Rule for AgricolaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("agricola")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "农场乐规则",
            &[
                ("游戏概述", &self.section_overview()),
                ("游戏组件", &self.section_components()),
                ("资源类型", &self.section_resources()),
                ("动物种类", &self.section_animals()),
                ("游戏回合", &self.section_rounds()),
                ("行动格类型", &self.section_actions()),
                ("农场建设", &self.section_farm_building()),
                ("房屋升级", &self.section_housing()),
                ("家庭成员", &self.section_family()),
                ("食物需求", &self.section_food()),
                ("收获阶段", &self.section_harvest()),
                ("计分系统", &self.section_scoring()),
                ("负分规则", &self.section_penalties()),
                ("游戏结束", &self.section_endgame()),
                ("策略要点", &self.section_strategy()),
                ("变体版本", &self.section_variants()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agricola_rules_basic() {
        let rules = AgricolaRules::new();
        assert_eq!(rules.metadata().name, "农场乐规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn agricola_has_resources() {
        let rules = AgricolaRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("木材") || explanation.contains("砖块"));
    }

    #[test]
    fn agricola_has_animals() {
        let rules = AgricolaRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("羊") || explanation.contains("牛"));
    }

    #[test]
    fn agricola_scoring() {
        let rules = AgricolaRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("得分") || explanation.contains("负分"));
    }

    #[test]
    fn agricola_has_origin() {
        let rules = AgricolaRules::new();
        assert_eq!(rules.metadata().origin, Some("德国".to_string()));
    }
}