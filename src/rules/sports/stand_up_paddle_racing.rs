//! 竞技桨板规则
//!
//! 竞技桨板(SUP Racing)是快速发展的水上运动，
//! 世界泳联正式竞赛项目。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 竞技桨板规则
pub struct StandUpPaddleRacingRules {
    metadata: RuleMetadata,
}

impl StandUpPaddleRacingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("竞技桨板规则", "世界泳联桨板竞赛规则")
                .with_origin("World Aquatics / ICF")
                .with_tags(vec!["体育".into(), "水上".into(), "桨板".into()]),
        }
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "男子短距离竞速(200米)",
            "女子短距离竞速(200米)",
            "男子长距离竞速(5000米)",
            "女子长距离竞速(5000米)",
            "男子技术赛",
            "女子技术赛",
            "混合接力赛",
        ]
    }

    /// 板型分类
    pub fn board_types(&self) -> Vec<&'static str> {
        vec![
            "竞赛板: 12.6英尺(3.84米)",
            "竞赛板: 14英尺(4.27米)",
            "技术板: 较短、更灵活",
            "充气板: 允许在特定赛事",
            "宽度限制: 根据身高规定",
            "板材认证要求",
        ]
    }

    /// 比赛距离
    pub fn race_distances(&self) -> Vec<&'static str> {
        vec![
            "短距离: 200-400米",
            "中距离: 1000-2000米",
            "长距离: 5000-10000米",
            "马拉松: 15-20公里",
            "技术赛: 1-2公里绕标",
            "接力赛: 每人500米",
        ]
    }

    /// 竞赛规则
    pub fn racing_rules(&self) -> Vec<&'static str> {
        vec![
            "站立姿势比赛",
            "单桨划水",
            "绕标规则: 按规定方向",
            "起航: 集体或分组",
            "终点判定: 完整通过终点线",
            "可跪姿短暂休息",
        ]
    }

    /// 起航规则
    pub fn start_rules(&self) -> Vec<&'static str> {
        vec![
            "起航信号: 5分钟预告",
            "分组起航: 每组最多8人",
            "抢航处罚: 加时或取消",
            "起航位置: 随机抽签",
            "稳定姿势待命",
            "听到信号后起航",
        ]
    }

    /// 评分系统
    pub fn scoring_system(&self) -> Vec<&'static str> {
        vec![
            "计时排名制",
            "精确计时: 0.01秒",
            "积分制: 根据名次",
            "总积分累计排名",
            "可丢弃最差一轮",
            "金牌轮双倍积分",
        ]
    }

    /// 犯规与处罚
    pub fn penalties(&self) -> Vec<&'static str> {
        vec![
            "抢航: 加时惩罚",
            "绕标违规: 取消成绩",
            "碰撞: 评分惩罚",
            "违规姿势: 取消成绩",
            "使用禁用装备: 取消",
            "干扰他人: 取消资格",
        ]
    }

    /// 装备要求
    pub fn equipment_requirements(&self) -> Vec<&'static str> {
        vec![
            "桨板符合级别标准",
            "桨叶长度: 身高+8英寸",
            " leash绳强制佩戴",
            "救生衣强制穿戴(长距离)",
            "禁止使用风帆辅助",
            "装备认证标记",
        ]
    }

    /// 安全要求
    pub fn safety_requirements(&self) -> Vec<&'static str> {
        vec![
            "水温不低于15°C",
            "救援艇待命",
            "医疗救护设施",
            "通信联络设备",
            "选手安全教育",
            "恶劣天气预案",
        ]
    }

    /// 参赛资格
    pub fn participation_requirements(&self) -> Vec<&'static str> {
        vec![
            "年龄限制: 14岁以上",
            "通过资格赛选拔",
            "国家级协会认证",
            "体检合格证明",
            "保险证明",
            "级别认证",
        ]
    }
}

impl Default for StandUpPaddleRacingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for StandUpPaddleRacingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("stand_up_paddle_racing")
    }

    fn explain(&self) -> String {
        format!(
            "【竞技桨板规则】\n\n\
            比赛项目:\n{}\n\n\
            板型分类:\n{}\n\n\
            竞赛规则:\n{}\n",
            self.events()
                .iter()
                .map(|e| format!("  • {}", e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.board_types()
                .iter()
                .map(|b| format!("  • {}", b))
                .collect::<Vec<_>>()
                .join("\n"),
            self.racing_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stand_up_paddle_racing_rules_basic() {
        let rules = StandUpPaddleRacingRules::new();
        assert_eq!(rules.metadata().name, "竞技桨板规则");
        assert!(!rules.explain().is_empty());
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }

    #[test]
    fn stand_up_paddle_racing_events() {
        let rules = StandUpPaddleRacingRules::new();
        let events = rules.events();
        assert!(events.iter().any(|e| e.contains("竞速")));
        assert!(events.iter().any(|e| e.contains("技术")));
        assert!(events.len() >= 7);
    }

    #[test]
    fn stand_up_paddle_racing_distances() {
        let rules = StandUpPaddleRacingRules::new();
        let distances = rules.race_distances();
        assert!(distances.iter().any(|d| d.contains("短距离")));
        assert!(distances.iter().any(|d| d.contains("长距离")));
        assert!(distances.len() >= 6);
    }
}