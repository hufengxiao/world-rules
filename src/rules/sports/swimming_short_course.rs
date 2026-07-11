//! 短池游泳规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 短池游泳规则
pub struct SwimmingShortCourseRules {
    metadata: RuleMetadata,
}

impl SwimmingShortCourseRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("短池游泳规则", "25米短池游泳比赛规则")
                .with_origin("FINA")
                .with_tags(vec!["体育".into(), "游泳".into(), "短池".into()]),
        }
    }

    /// 泳池规格
    pub fn pool_specifications(&self) -> Vec<&'static str> {
        vec![
            "长度: 25米 (短池)",
            "宽度: 至少18米 (6条泳道)",
            "深度: 至少1.0米",
            "泳道宽度: 2.5米",
            "水温: 25-28°C",
            "转身区: 每端5米标记",
        ]
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "自由泳: 50m/100m/200m/400m",
            "仰泳: 50m/100m/200m",
            "蛙泳: 50m/100m/200m",
            "蝶泳: 50m/100m/200m",
            "个人混合泳: 100m/200m/400m",
            "自由泳接力: 4×50m/4×100m",
            "混合泳接力: 4×50m/4×100m",
        ]
    }

    /// 转身规则
    pub fn turn_rules(&self) -> Vec<&'static str> {
        vec![
            "自由泳: 允许滚翻转身",
            "仰泳: 可翻转后转身",
            "蛙泳: 必须双手触壁",
            "蝶泳: 必须双手触壁",
            "混合泳: 按各泳姿规则",
            "转身后可潜泳15米",
            "转身时脚必须触壁",
        ]
    }

    /// 世界纪录
    pub fn records(&self) -> Vec<&'static str> {
        vec![
            "短池世界纪录: FINA认证",
            "短池洲际纪录",
            "短池国家纪录",
            "短池年龄组纪录",
            "世界纪录奖金: $15,000",
        ]
    }

    /// 与长池区别
    pub fn differences_from_long_course(&self) -> Vec<&'static str> {
        vec![
            "转身次数更多",
            "转身技术更重要",
            "比赛时间通常更快",
            "项目距离更短",
            "赛季通常在冬季",
            "更适合室内场馆",
        ]
    }

    /// 赛事类型
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            "短池世锦赛: 每两年",
            "短池世界杯: 年度系列赛",
            "洲际短池锦标赛",
            "国家短池锦标赛",
            "俱乐部短池联赛",
        ]
    }
}

impl Default for SwimmingShortCourseRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SwimmingShortCourseRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("swimming_short_course")
    }

    fn explain(&self) -> String {
        format!(
            "【短池游泳规则】\n\n\
            泳池规格:\n{}\n\n\
            比赛项目:\n{}\n\n\
            转身规则:\n{}\n\n\
            与长池区别:\n{}",
            self.pool_specifications()
                .iter()
                .map(|p| format!("  • {}", p))
                .collect::<Vec<_>>()
                .join("\n"),
            self.events()
                .iter()
                .map(|e| format!("  • {}", e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.turn_rules()
                .iter()
                .map(|t| format!("  • {}", t))
                .collect::<Vec<_>>()
                .join("\n"),
            self.differences_from_long_course()
                .iter()
                .map(|d| format!("  • {}", d))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}
