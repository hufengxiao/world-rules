//! 龙舟世界锦标赛规则
//!
//! 龙舟是中国传统水上运动，
//! 现已成为国际龙舟联合会(IDBF)正式竞赛项目。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 龙舟世界锦标赛规则
pub struct DragonBoatWorldChampionshipRules {
    metadata: RuleMetadata,
}

impl DragonBoatWorldChampionshipRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("龙舟世界锦标赛规则", "国际龙舟联合会世界锦标赛规则")
                .with_origin("IDBF")
                .with_tags(vec![
                    "体育".into(),
                    "水上".into(),
                    "龙舟".into(),
                    "中国传统".into(),
                ]),
        }
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "男子标准龙舟200米",
            "女子标准龙舟200米",
            "混合标准龙舟200米",
            "男子标准龙舟500米",
            "女子标准龙舟500米",
            "混合标准龙舟500米",
            "男子标准龙舟1000米",
            "女子标准龙舟1000米",
            "小龙舟200米",
            "小龙舟500米",
        ]
    }

    /// 龙舟规格
    pub fn boat_specifications(&self) -> Vec<&'static str> {
        vec![
            "标准龙舟: 12.4米长",
            "标准龙舟: 1.1米宽",
            "标准龙舟: 20名划手",
            "小龙舟: 9.6米长",
            "小龙舟: 10名划手",
            "重量限制: 标准龙舟240公斤",
        ]
    }

    /// 人员配置
    pub fn crew_composition(&self) -> Vec<&'static str> {
        vec![
            "标准龙舟: 20划手",
            "标准龙舟: 1鼓手",
            "标准龙舟: 1舵手",
            "小龙舟: 10划手",
            "小龙舟: 1鼓手",
            "小龙舟: 1舵手",
        ]
    }

    /// 比赛距离
    pub fn race_distances(&self) -> Vec<&'static str> {
        vec![
            "短距离: 200米",
            "中距离: 500米",
            "长距离: 1000米",
            "马拉松: 2000米以上",
            "绕标赛: 特殊赛道",
            "赛道宽度要求",
        ]
    }

    /// 竞赛规则
    pub fn racing_rules(&self) -> Vec<&'static str> {
        vec![
            "起航: 固定起航系统",
            "赛道: 6-8条航道",
            "计时精确: 0.01秒",
            "终点判定: 完整通过终点线",
            "鼓手指挥节奏",
            "舵手控制方向",
        ]
    }

    /// 评分系统
    pub fn scoring_system(&self) -> Vec<&'static str> {
        vec![
            "计时排名制",
            "积分制: 根据名次",
            "总积分累计排名",
            "可丢弃最差一轮成绩",
            "奖牌: 金、银、铜",
            "团体总分排名",
        ]
    }

    /// 犯规与处罚
    pub fn penalties(&self) -> Vec<&'static str> {
        vec![
            "抢航: 加时或取消",
            "越道: 取消成绩",
            "碰撞: 评分惩罚",
            "人员违规: 取消资格",
            "装备违规: 取消资格",
            "超时: 取消成绩",
        ]
    }

    /// 装备要求
    pub fn equipment_requirements(&self) -> Vec<&'static str> {
        vec![
            "龙舟符合IDBF标准",
            "桨叶规格限制",
            "救生衣强制穿戴",
            "鼓和鼓槌规格",
            "龙舟装饰规定",
            "安全绳要求",
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

    /// 文化元素
    pub fn cultural_elements(&self) -> Vec<&'static str> {
        vec![
            "龙舟装饰: 龙头龙尾",
            "传统仪式: 点睛仪式",
            "鼓声节奏: 传统鼓法",
            "端午节传统",
            "纪念屈原文化",
            "龙舟文化节",
        ]
    }

    /// 参赛资格
    pub fn qualification_requirements(&self) -> Vec<&'static str> {
        vec![
            "国家级协会认证",
            "地区资格赛选拔",
            "世界排名积分",
            "上届成绩保障名额",
            "主办国自动资格",
            "年龄分组规定",
        ]
    }
}

impl Default for DragonBoatWorldChampionshipRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for DragonBoatWorldChampionshipRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("dragon_boat_world_championship")
    }

    fn explain(&self) -> String {
        format!(
            "【龙舟世界锦标赛规则】\n\n\
            比赛项目:\n{}\n\n\
            龙舟规格:\n{}\n\n\
            文化元素:\n{}\n",
            self.events()
                .iter()
                .map(|e| format!("  • {}", e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.boat_specifications()
                .iter()
                .map(|b| format!("  • {}", b))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cultural_elements()
                .iter()
                .map(|c| format!("  • {}", c))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dragon_boat_world_championship_rules_basic() {
        let rules = DragonBoatWorldChampionshipRules::new();
        assert_eq!(rules.metadata().name, "龙舟世界锦标赛规则");
        assert!(!rules.explain().is_empty());
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }

    #[test]
    fn dragon_boat_world_championship_events() {
        let rules = DragonBoatWorldChampionshipRules::new();
        let events = rules.events();
        assert!(events.iter().any(|e| e.contains("标准龙舟")));
        assert!(events.iter().any(|e| e.contains("小龙舟")));
        assert!(events.len() >= 10);
    }

    #[test]
    fn dragon_boat_world_championship_specs() {
        let rules = DragonBoatWorldChampionshipRules::new();
        let specs = rules.boat_specifications();
        assert!(specs.iter().any(|s| s.contains("12.4米")));
        assert!(specs.iter().any(|s| s.contains("20名")));
        assert!(specs.len() >= 6);
    }

    #[test]
    fn dragon_boat_world_championship_culture() {
        let rules = DragonBoatWorldChampionshipRules::new();
        let culture = rules.cultural_elements();
        assert!(culture.iter().any(|c| c.contains("龙头")));
        assert!(culture.iter().any(|c| c.contains("屈原")));
        assert!(culture.len() >= 6);
    }
}
