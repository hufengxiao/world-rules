//! 残疾人雪橇冰球规则
//!
//! 残疾人雪橇冰球是残奥会冬季项目，又称雪橇曲棍球。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 残疾人雪橇冰球规则
pub struct ParaSledgeHockeyRules {
    metadata: RuleMetadata,
}

impl ParaSledgeHockeyRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("残疾人雪橇冰球规则", "残疾人雪橇冰球比赛规则")
                .with_origin("IPC/IIHF")
                .with_tags(vec!["体育".into(), "冰球".into(), "残奥".into()]),
        }
    }

    /// 运动分级
    pub fn classification(&self) -> Vec<&'static str> {
        vec![
            "最低残疾: 下肢残疾",
            "分级范围: 不分级别",
            "残疾类型: 截肢、脊髓损伤等",
            "点数系统: 无点数限制",
            "分级评估: 功能测试",
            "性别分组: 男女混合",
        ]
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "残奥会: 单项",
            "世界锦标赛A组",
            "世界锦标赛B组",
            "洲际锦标赛",
            "比赛制: 3局×15分钟",
            "加时赛: 突然死亡",
        ]
    }

    /// 场地规格
    pub fn rink(&self) -> Vec<&'static str> {
        vec![
            "冰场: 60×30米",
            "球门: 1.83×1.22米",
            "球门区: 标准",
            "争球点: 9个",
            "板墙: 标准高度",
            "冰面: 专业冰场",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "雪橇: 专用冰橇",
            "冰刀: 双刀设计",
            "球杆: 双头设计",
            "冰球: 标准冰球",
            "护具: 头盔、护甲",
            "手套: 强制",
            "守门员装备: 专用",
        ]
    }

    /// 技术规则
    pub fn technique(&self) -> Vec<&'static str> {
        vec![
            "每队上场: 6人",
            "换人: 随时换人",
            "越位: 有越位规则",
            "死球: 有死球规则",
            "冲撞: 禁止冲撞",
            "球杆: 双头推冰",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "冲撞犯规",
            "举杆过高",
            "钩人",
            "绊人",
            "肘击",
            "粗鲁行为",
            "雪橇犯规",
        ]
    }

    /// 适应性规则
    pub fn adaptations(&self) -> Vec<&'static str> {
        vec![
            "雪橇代替滑冰",
            "双刀设计",
            "双头球杆",
            "禁止站立冲撞",
            "雪橇技术要求",
            "守门员特殊装备",
        ]
    }

    /// 参赛资格
    pub fn eligibility(&self) -> Vec<&'static str> {
        vec![
            "IPC分级认证",
            "最低残疾标准",
            "国际冰联注册",
            "滑冰能力证明",
            "体检合格证明",
        ]
    }
}

impl Default for ParaSledgeHockeyRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ParaSledgeHockeyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("para_sledge_hockey")
    }

    fn explain(&self) -> String {
        format!(
            "【残疾人雪橇冰球规则】\n\n\
            运动分级:\n{}\n\n\
            比赛项目:\n{}\n\n\
            场地规格:\n{}\n\n\
            技术规则:\n{}",
            self.classification()
                .iter()
                .map(|c| format!("  • {}", c))
                .collect::<Vec<_>>()
                .join("\n"),
            self.events()
                .iter()
                .map(|e| format!("  • {}", e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.rink()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.technique()
                .iter()
                .map(|t| format!("  • {}", t))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_para_sledge_hockey_rules_basic() {
        let rules = ParaSledgeHockeyRules::new();
        assert_eq!(rules.metadata().name, "残疾人雪橇冰球规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_para_sledge_hockey_classification() {
        let rules = ParaSledgeHockeyRules::new();
        let classification = rules.classification();
        assert!(classification.iter().any(|c| c.contains("下肢")));
        assert!(classification.iter().any(|c| c.contains("残疾")));
        assert!(classification.len() >= 4);
    }

    #[test]
    fn test_para_sledge_hockey_events() {
        let rules = ParaSledgeHockeyRules::new();
        let events = rules.events();
        assert!(events.iter().any(|e| e.contains("残奥会")));
        assert!(events.iter().any(|e| e.contains("世界锦标赛")));
        assert!(events.len() >= 4);
    }

    #[test]
    fn test_para_sledge_hockey_equipment() {
        let rules = ParaSledgeHockeyRules::new();
        let equipment = rules.equipment();
        assert!(equipment.iter().any(|e| e.contains("雪橇")));
        assert!(equipment.iter().any(|e| e.contains("球杆")));
        assert!(equipment.len() >= 4);
    }

    #[test]
    fn test_para_sledge_hockey_category() {
        let rules = ParaSledgeHockeyRules::new();
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }
}
