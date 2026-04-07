//! 滑雪规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 滑雪项目类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkiingType {
    /// 高山滑雪
    Alpine,
    /// 越野滑雪
    CrossCountry,
    /// 跳台滑雪
    SkiJumping,
    /// 自由式滑雪
    Freestyle,
    /// 单板滑雪
    Snowboard,
}

impl SkiingType {
    pub fn name(&self) -> &'static str {
        match self {
            SkiingType::Alpine => "高山滑雪",
            SkiingType::CrossCountry => "越野滑雪",
            SkiingType::SkiJumping => "跳台滑雪",
            SkiingType::Freestyle => "自由式滑雪",
            SkiingType::Snowboard => "单板滑雪",
        }
    }
}

/// 滑雪规则
pub struct SkiingRules {
    metadata: RuleMetadata,
}

impl SkiingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "滑雪规则",
                "FIS 国际滑雪联合会标准规则"
            )
            .with_origin("FIS")
            .with_tags(vec!["体育".into(), "滑雪".into()]),
        }
    }

    /// 高山滑雪项目
    pub fn alpine_events(&self) -> Vec<&'static str> {
        vec![
            "滑降 (Downhill): 最快速度下坡",
            "超级大回转 (Super-G): 高速过弯",
            "大回转 (Giant Slalom): 中等弯道",
            "回转 (Slalom): 窄弯道",
            "全能 (Combined): 滑降+回转",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "佩戴头盔",
            "注意雪道标识",
            "控制速度",
            "优先让下方滑雪者",
            "不在雪道中间停留",
            "保持安全距离",
        ]
    }

    /// 雪道等级
    pub fn slope_ratings(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("绿色圆圈", "初级道(最简单)"),
            ("蓝色方块", "中级道"),
            ("黑色菱形", "高级道"),
            ("双黑菱形", "专家道(最难)"),
        ]
    }

    /// 越野滑雪技术
    pub fn cross_country_techniques(&self) -> Vec<&'static str> {
        vec![
            "传统式: 直线滑行",
            "自由式: 滑冰式动作",
            "交替滑行",
            "同时推进",
            "登坡技术",
        ]
    }

    /// 跳台滑雪评分
    pub fn ski_jumping_scoring(&self) -> Vec<&'static str> {
        vec![
            "距离分: 根据飞行距离",
            "姿态分: 5名裁判打分",
            "去掉最高分和最低分",
            "风速调整",
            "满分: 每跳最高120分",
        ]
    }
}

impl Default for SkiingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SkiingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("skiing")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        let ratings = self.slope_ratings();
        format!(
            "【滑雪规则】\n\n\
            高山滑雪项目:\n{}\n\n\
            雪道等级:\n{}\n\n\
            安全规则:\n{}\n\n\
            越野滑雪技术:\n{}\n\n\
            跳台滑雪评分:\n{}\n",
            self.alpine_events().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            ratings.iter().map(|(r, d)| format!("  • {}: {}", r, d)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.cross_country_techniques().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.ski_jumping_scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skiing_rules() {
        let rules = SkiingRules::new();
        assert!(!rules.alpine_events().is_empty());
    }
}