//! 赛艇世界杯规则
//!
//! 赛艇世界杯是世界赛艇联合会(FISA)最高级别系列赛。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 赛艇世界杯规则
pub struct RowingWorldCupRules {
    metadata: RuleMetadata,
}

impl RowingWorldCupRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("赛艇世界杯规则", "世界赛艇联合会世界杯系列赛规则")
                .with_origin("FISA")
                .with_tags(vec!["体育".into(), "水上".into(), "赛艇".into()]),
        }
    }

    /// 联赛结构
    pub fn series_structure(&self) -> Vec<&'static str> {
        vec![
            "世界杯系列赛: 3站比赛",
            "第一站: 4-5月",
            "第二站: 5-6月",
            "第三站: 6-7月",
            "世界锦标赛: 8-9月",
            "积分累计排名",
        ]
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "男子单人双桨(M1x)",
            "女子单人双桨(W1x)",
            "男子双人双桨(M2x)",
            "女子双人双桨(W2x)",
            "男子四人双桨(M4x)",
            "女子四人双桨(W4x)",
            "男子八人单桨有舵手(M8+)",
            "女子八人单桨有舵手(W8+)",
            "男子轻量级双人双桨(LM2x)",
            "女子轻量级双人双桨(LW2x)",
            "混合双人双桨(Mix2x)",
        ]
    }

    /// 船艇类型
    pub fn boat_types(&self) -> Vec<&'static str> {
        vec![
            "单人艇(1x): 1名运动员",
            "双人艇(2x/2-): 2名运动员",
            "四人艇(4x/4-): 4名运动员",
            "八人艇(8+): 8名运动员+舵手",
            "双桨: 每侧一支桨",
            "单桨: 每侧一支桨(交替划)",
        ]
    }

    /// 比赛距离
    pub fn race_distances(&self) -> Vec<&'static str> {
        vec![
            "标准距离: 2000米",
            "青年比赛: 1500米",
            "短距离赛: 500米",
            "航道宽度: 12.5米",
            "6条航道",
            "计时精确: 0.01秒",
        ]
    }

    /// 竞赛规则
    pub fn racing_rules(&self) -> Vec<&'static str> {
        vec![
            "起航: 固定起航系统",
            "起航信号: 声音和视觉信号",
            "抢航处罚: 取消资格",
            "航道规则: 保持在指定航道",
            "终点判定: 完整通过终点线",
            "犯规判定规则",
        ]
    }

    /// 评分系统
    pub fn scoring_system(&self) -> Vec<&'static str> {
        vec![
            "积分制: 根据名次",
            "第1名: 10分",
            "第2名: 8分",
            "第3名: 6分",
            "第4名: 5分",
            "总积分累计排名",
        ]
    }

    /// 晋级规则
    pub fn advancement_rules(&self) -> Vec<&'static str> {
        vec![
            "预赛: 所有选手参加",
            "复赛: 补赛晋级机会",
            "半决赛: 前12名晋级",
            "决赛A: 前6名争夺奖牌",
            "决赛B: 7-12名排名",
            "积分累计制",
        ]
    }

    /// 犯规与处罚
    pub fn penalties(&self) -> Vec<&'static str> {
        vec![
            "抢航: 取消资格",
            "越道: 取消成绩或降级",
            "碰撞: 评分惩罚",
            "超时: 取消成绩",
            "违反重量限制: 取消",
            "违规装备: 取消资格",
        ]
    }

    /// 装备要求
    pub fn equipment_requirements(&self) -> Vec<&'static str> {
        vec![
            "船艇符合FISA标准",
            "桨叶规格限制",
            "船艇重量最低限制",
            "轻量级重量限制",
            "舵手重量最低要求",
            "安全设备要求",
        ]
    }

    /// 参赛资格
    pub fn qualification_requirements(&self) -> Vec<&'static str> {
        vec![
            "国家级协会认证",
            "世界排名或资格赛",
            "地区名额分配",
            "上届成绩保障名额",
            "主办国自动资格",
            "奥运积分关联",
        ]
    }

    /// 安全要求
    pub fn safety_requirements(&self) -> Vec<&'static str> {
        vec![
            "水温不低于10°C",
            "救援艇待命",
            "医疗救护设施",
            "通信联络设备",
            "天气监测系统",
            "恶劣天气预案",
        ]
    }
}

impl Default for RowingWorldCupRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for RowingWorldCupRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("rowing_world_cup")
    }

    fn explain(&self) -> String {
        format!(
            "【赛艇世界杯规则】\n\n\
            联赛结构:\n{}\n\n\
            比赛项目:\n{}\n\n\
            积分系统:\n{}\n",
            self.series_structure()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.events()
                .iter()
                .map(|e| format!("  • {}", e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.scoring_system()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rowing_world_cup_rules_basic() {
        let rules = RowingWorldCupRules::new();
        assert_eq!(rules.metadata().name, "赛艇世界杯规则");
        assert!(!rules.explain().is_empty());
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }

    #[test]
    fn rowing_world_cup_series_structure() {
        let rules = RowingWorldCupRules::new();
        let structure = rules.series_structure();
        assert!(structure.iter().any(|s| s.contains("世界杯")));
        assert!(structure.iter().any(|s| s.contains("3站")));
        assert!(structure.len() >= 6);
    }

    #[test]
    fn rowing_world_cup_events() {
        let rules = RowingWorldCupRules::new();
        let events = rules.events();
        assert!(events.iter().any(|e| e.contains("单人")));
        assert!(events.iter().any(|e| e.contains("双人")));
        assert!(events.iter().any(|e| e.contains("八人")));
        assert!(events.len() >= 11);
    }

    #[test]
    fn rowing_world_cup_scoring() {
        let rules = RowingWorldCupRules::new();
        let scoring = rules.scoring_system();
        assert!(scoring.iter().any(|s| s.contains("积分")));
        assert!(scoring.iter().any(|s| s.contains("10分")));
        assert!(scoring.len() >= 6);
    }
}
