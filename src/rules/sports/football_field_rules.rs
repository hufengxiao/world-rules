//! 球场场地规则详解

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 球场区域类型
#[derive(Debug, Clone, Copy)]
pub enum FieldArea {
    /// 禁区
    PenaltyArea,
    /// 小禁区
    GoalArea,
    /// 角球弧
    CornerArc,
    /// 中圈
    CenterCircle,
    /// 罚球点
    PenaltySpot,
}

/// 球场场地规则详解
pub struct FootballFieldRules {
    metadata: RuleMetadata,
}

impl FootballFieldRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("球场场地规则详解", "足球球场规格和场地的完整规则")
                .with_origin("FIFA")
                .with_tags(vec!["体育".into(), "足球".into(), "场地".into()]),
        }
    }

    /// 球场基本规格
    pub fn basic_dimensions(&self) -> Vec<&'static str> {
        vec![
            "长度:90-120米(国际比赛100-110米)",
            "宽度:45-90米(国际比赛64-75米)",
            "国际标准:105×68米",
            "矩形形状",
            "草地或人造草",
            "平坦无障碍物",
        ]
    }

    /// 球门规格
    pub fn goal_dimensions(&self) -> Vec<&'static str> {
        vec![
            "宽度:7.32米",
            "高度:2.44米",
            "门柱宽度:不超过12厘米",
            "横梁宽度:不超过12厘米",
            "门网深度:至少1.5米",
            "白色门柱和横梁",
        ]
    }

    /// 禁区规格
    pub fn penalty_area_specs(&self) -> Vec<&'static str> {
        vec![
            "长度:40.32米(16.5米×2+7.32米)",
            "宽度:16.5米",
            "从门柱向外延伸",
            "点球点距球门11米",
            "禁区弧半径9.15米",
            "禁区线属于禁区",
        ]
    }

    /// 小禁区规格
    pub fn goal_area_specs(&self) -> Vec<&'static str> {
        vec![
            "长度:18.32米(5.5米×2+7.32米)",
            "宽度:5.5米",
            "从门柱向外延伸",
            "门球执行区域",
            "门将活动区域",
            "小禁区线属于小禁区",
        ]
    }

    /// 中圈规格
    pub fn center_circle_specs(&self) -> Vec<&'static str> {
        vec![
            "半径:9.15米",
            "中心点标记",
            "开球执行点",
            "对手必须退至圈外",
            "中场线穿过中心",
            "白色标记线",
        ]
    }

    /// 角球弧规格
    pub fn corner_arc_specs(&self) -> Vec<&'static str> {
        vec![
            "半径:1米",
            "四个角球弧",
            "角旗杆高度:不低于1.5米",
            "角旗杆顶部非尖锐",
            "角球执行区域",
            "白色标记弧",
        ]
    }

    /// 场地标记线
    pub fn field_markings(&self) -> Vec<&'static str> {
        vec![
            "所有线宽度不超过12厘米",
            "边界线属于场地",
            "中场线分割场地",
            "两半场对称",
            "白色标记线",
            "清晰可辨认",
        ]
    }

    /// 场地安全要求
    pub fn safety_requirements(&self) -> Vec<&'static str> {
        vec![
            "场地平整",
            "无障碍物",
            "安全距离",
            "急救通道",
            "观众隔离",
            "天气条件适合",
        ]
    }

    /// 替补席区域
    pub fn technical_area(&self) -> Vec<&'static str> {
        vec![
            "教练席位置",
            "替补席位置",
            "医疗人员区域",
            "VAR裁判区域",
            "指定距离",
            "技术区域限制",
        ]
    }

    /// 计算禁区面积(平方米)
    pub fn calculate_penalty_area(&self) -> f32 {
        40.32 * 16.5 // 约666平方米
    }

    /// 计算球场总面积(平方米)
    pub fn calculate_total_area(&self, length: f32, width: f32) -> f32 {
        length * width
    }

    /// 判定点是否在禁区内
    pub fn is_in_penalty_area(&self, x: f32, y: f32, is_home_side: bool) -> bool {
        // 简化判定：y坐标在16.5米范围内且x接近禁区边界
        let penalty_width = 16.5;
        let penalty_length = 40.32;
        let goal_width = 7.32;

        // 禁区从门柱延伸，中心在球门中心
        if is_home_side {
            y >= 0.0
                && y <= penalty_width
                && x >= 0.0
                && x <= penalty_length / 2.0 + goal_width / 2.0
        } else {
            y >= 0.0 && y <= penalty_width && x >= (105.0 - penalty_length / 2.0 - goal_width / 2.0)
        }
    }
}

impl Default for FootballFieldRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FootballFieldRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_field")
    }

    fn explain(&self) -> String {
        format!(
            "【球场场地规则详解】\n\n\
            基本规格:\n{}\n\n\
            球门规格:\n{}\n\n\
            禁区规格:\n{}\n\n\
            中圈规格:\n{}\n\n\
            场地标记线:\n{}\n",
            self.basic_dimensions()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.goal_dimensions()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.penalty_area_specs()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.center_circle_specs()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.field_markings()
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
    fn test_basic_dimensions() {
        let rules = FootballFieldRules::new();
        let dims = rules.basic_dimensions();
        assert!(dims.contains(&"国际标准:105×68米"));
        assert!(dims.contains(&"草地或人造草"));
    }

    #[test]
    fn test_goal_dimensions() {
        let rules = FootballFieldRules::new();
        let goal = rules.goal_dimensions();
        assert!(goal.contains(&"宽度:7.32米"));
        assert!(goal.contains(&"高度:2.44米"));
    }

    #[test]
    fn test_penalty_area_specs() {
        let rules = FootballFieldRules::new();
        let specs = rules.penalty_area_specs();
        assert!(specs.contains(&"点球点距球门11米"));
        assert!(specs.contains(&"禁区弧半径9.15米"));
    }

    #[test]
    fn test_calculate_penalty_area() {
        let rules = FootballFieldRules::new();
        let area = rules.calculate_penalty_area();
        assert!(area > 600.0 && area < 700.0);
    }

    #[test]
    fn test_calculate_total_area() {
        let rules = FootballFieldRules::new();
        let area = rules.calculate_total_area(105.0, 68.0);
        assert_eq!(area, 7140.0); // 105×68 = 7140平方米
    }

    #[test]
    fn test_center_circle_specs() {
        let rules = FootballFieldRules::new();
        let circle = rules.center_circle_specs();
        assert!(circle.contains(&"半径:9.15米"));
        assert!(circle.contains(&"开球执行点"));
    }

    #[test]
    fn test_metadata() {
        let rules = FootballFieldRules::new();
        assert_eq!(rules.metadata().name, "球场场地规则详解");
        assert_eq!(rules.category(), RuleCategory::sports("football_field"));
    }
}
