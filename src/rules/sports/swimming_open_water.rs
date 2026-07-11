//! 公开水域游泳规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 公开水域游泳规则
pub struct SwimmingOpenWaterRules {
    metadata: RuleMetadata,
}

impl SwimmingOpenWaterRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("公开水域游泳规则", "公开水域游泳比赛规则")
                .with_origin("FINA")
                .with_tags(vec!["体育".into(), "游泳".into(), "公开水域".into()]),
        }
    }

    /// 比赛距离
    pub fn distances(&self) -> Vec<&'static str> {
        vec![
            "短距离: 1.5公里",
            "中距离: 5公里",
            "长距离: 10公里 (奥运项目)",
            "马拉松: 25公里",
            "超长距离: 自定义",
        ]
    }

    /// 比赛场地
    pub fn venues(&self) -> Vec<&'static str> {
        vec!["海洋游泳", "湖泊游泳", "河流游泳", "人工湖", "水库", "港湾"]
    }

    /// 安全要求
    pub fn safety_requirements(&self) -> Vec<&'static str> {
        vec![
            "水温下限: 16°C (5km以下)",
            "水温下限: 18°C (10km以上)",
            "水温上限: 31°C",
            "安全艇: 每运动员一艘",
            "医疗艇: 现场待命",
            "潜水救援队",
            "水温监测系统",
            "GPS追踪设备",
        ]
    }

    /// 比赛规则
    pub fn race_rules(&self) -> Vec<&'static str> {
        vec![
            "站立: 允许，但不能前进",
            "接触: 禁止故意接触",
            "跟随: 允许跟随划水",
            "补给: 通过补给站",
            "转身: 绕过浮标",
            "终点: 触摸终点板或上岸",
        ]
    }

    /// 犯规行为
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "故意阻碍对手",
            "拉扯其他运动员",
            "使用辅助器具",
            "未绕过浮标",
            "接受非法补给",
            "脱离指定路线",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "泳帽: 必须佩戴 (带编号)",
            "泳镜: 允许",
            "泳衣: 遵守FINA规定",
            "防鲨网: 视场地而定",
            "禁止: 潜水服 (水温>18°C)",
            "禁止: 脚蹼、划水板",
        ]
    }

    /// 赛事类型
    pub fn event_types(&self) -> Vec<&'static str> {
        vec![
            "奥运马拉松游泳: 10公里",
            "世锦赛: 5km/10km/25公里",
            "世界杯系列赛",
            "洲际锦标赛",
            "穿越英吉利海峡",
            "曼哈顿岛马拉松游泳",
        ]
    }
}

impl Default for SwimmingOpenWaterRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SwimmingOpenWaterRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("swimming_open_water")
    }

    fn explain(&self) -> String {
        format!(
            "【公开水域游泳规则】\n\n\
            比赛距离:\n{}\n\n\
            安全要求:\n{}\n\n\
            比赛规则:\n{}\n\n\
            装备要求:\n{}",
            self.distances()
                .iter()
                .map(|d| format!("  • {}", d))
                .collect::<Vec<_>>()
                .join("\n"),
            self.safety_requirements()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.race_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.equipment()
                .iter()
                .map(|e| format!("  • {}", e))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}
