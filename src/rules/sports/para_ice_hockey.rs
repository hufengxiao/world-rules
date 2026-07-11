//! 残疾人冰球规则
//!
//! 残疾人冰球又称雪橇冰球，是残奥会冬季项目。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 残疾人冰球规则
pub struct ParaIceHockeyRules {
    metadata: RuleMetadata,
}

impl ParaIceHockeyRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("残疾人冰球规则", "残疾人冰球（雪橇冰球）比赛规则")
                .with_origin("IPC/IIHF")
                .with_tags(vec![
                    "体育".into(),
                    "冰球".into(),
                    "残奥".into(),
                    "冬季".into(),
                ]),
        }
    }

    /// 运动分级
    pub fn classification(&self) -> Vec<&'static str> {
        vec![
            "最低残疾要求: 下肢残疾",
            "功能分级: 不分级别",
            "残疾类型: 截肢、脊髓损伤等",
            "性别混合: 允许",
            "最低功能标准测试",
            "队伍人数: 男女混合",
        ]
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "残奥会: 混合团体",
            "世界锦标赛: A/B组",
            "洲际锦标赛",
            "国内联赛",
            "比赛制: 3节×15分钟",
            "队伍人数: 6人上场",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "雪橇: 双刃冰刀",
            "球杆: 双端球杆",
            "冰球: 标准冰球",
            "护具: 标准冰球护具",
            "头盔: 必须佩戴",
            "手套: 标准",
            "禁止: 非认证器材",
        ]
    }

    /// 比赛规则
    pub fn gameplay(&self) -> Vec<&'static str> {
        vec![
            "雪橇移动: 推动滑行",
            "球杆: 挑球和推动",
            "比赛时间: 3节×15分钟",
            "换人: 随时换人",
            "得分: 球进网得分",
            "冰场: 标准冰球场",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "雪橇冲撞",
            "非法身体接触",
            "球杆违规",
            "延误比赛",
            "绊人",
            "肘击/高杆",
        ]
    }

    /// 特殊规则
    pub fn special_rules(&self) -> Vec<&'static str> {
        vec![
            "T形球杆: 挑球端",
            "双刃雪橇: 平衡稳定",
            "冰面要求: 平整",
            "换人区: 雪橇进出",
            "门将: 特殊装备",
            "罚球区: 雪橇罚球",
        ]
    }

    /// 参赛资格
    pub fn eligibility(&self) -> Vec<&'static str> {
        vec![
            "IPC分级认证",
            "下肢残疾标准",
            "功能测试",
            "IIHF雪橇冰球执照",
            "国家队注册",
        ]
    }
}

impl Default for ParaIceHockeyRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ParaIceHockeyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("para_ice_hockey")
    }

    fn explain(&self) -> String {
        format!(
            "【残疾人冰球规则】\n\n\
            运动分级:\n{}\n\n\
            比赛项目:\n{}\n\n\
            装备要求:\n{}\n\n\
            比赛规则:\n{}",
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
            self.equipment()
                .iter()
                .map(|eq| format!("  • {}", eq))
                .collect::<Vec<_>>()
                .join("\n"),
            self.gameplay()
                .iter()
                .map(|g| format!("  • {}", g))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_para_ice_hockey_rules_basic() {
        let rules = ParaIceHockeyRules::new();
        assert_eq!(rules.metadata().name, "残疾人冰球规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_para_ice_hockey_classification() {
        let rules = ParaIceHockeyRules::new();
        let classification = rules.classification();
        assert!(classification.iter().any(|c| c.contains("下肢")));
        assert!(classification.len() >= 4);
    }

    #[test]
    fn test_para_ice_hockey_events() {
        let rules = ParaIceHockeyRules::new();
        let events = rules.events();
        assert!(events.iter().any(|e| e.contains("残奥会")));
        assert!(events.iter().any(|e| e.contains("团体")));
        assert!(events.len() >= 4);
    }

    #[test]
    fn test_para_ice_hockey_equipment() {
        let rules = ParaIceHockeyRules::new();
        let equipment = rules.equipment();
        assert!(equipment.iter().any(|e| e.contains("雪橇")));
        assert!(equipment.iter().any(|e| e.contains("球杆")));
        assert!(equipment.len() >= 4);
    }

    #[test]
    fn test_para_ice_hockey_gameplay() {
        let rules = ParaIceHockeyRules::new();
        let gameplay = rules.gameplay();
        assert!(gameplay.iter().any(|g| g.contains("雪橇")));
        assert!(gameplay.iter().any(|g| g.contains("比赛时间")));
        assert!(gameplay.len() >= 4);
    }

    #[test]
    fn test_para_ice_hockey_category() {
        let rules = ParaIceHockeyRules::new();
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }
}
