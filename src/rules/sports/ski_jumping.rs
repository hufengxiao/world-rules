//! 跳台滑雪规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 跳台滑雪规则
pub struct SkiJumpingRules {
    metadata: RuleMetadata,
}

impl SkiJumpingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "跳台滑雪规则",
                "跳台滑雪比赛基本规则"
            )
            .with_origin("挪威")
            .with_tags(vec!["体育".into(), "冬季".into()]),
        }
    }

    /// 跳台类型
    pub fn hill_types(&self) -> Vec<&'static str> {
        vec![
            "标准台: K点75-99米",
            "大台: K点100-169米",
            "飞行台: K点170米以上",
            "迷你跳台: 训练用",
            "女子标准台",
        ]
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "个人赛",
            "团体赛",
            "大型飞行赛",
            "世界杯系列",
            "奥运会项目",
        ]
    }

    /// 评分标准
    pub fn scoring_criteria(&self) -> Vec<&'static str> {
        vec![
            "距离分: 飞行距离",
            "姿态分: 飞行姿势",
            "风速修正",
            "出发位置调整",
            "总分计算",
        ]
    }

    /// 技术要求
    pub fn technique(&self) -> Vec<&'static str> {
        vec![
            "助滑速度",
            "起跳时机",
            "飞行姿势",
            "V形姿态",
            "着陆技术",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "滑雪板: 长度为身高146%",
            "滑雪服: 气动设计",
            "头盔: 安全保护",
            "护目镜: 视力保护",
            "手套和靴子",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "每轮2次跳跃",
            "决赛2次跳跃",
            "总分最高获胜",
            "资格赛晋级",
            "出发顺序",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "跳台安全检查",
            "天气条件监控",
            "着陆区安全",
            "医疗支持",
            "训练要求",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "摔倒",
            "着陆不稳",
            "犯规起跳",
            "装备违规",
            "超时",
        ]
    }
}

impl Default for SkiJumpingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SkiJumpingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("ski_jumping")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【跳台滑雪规则】\n\n\
            跳台类型:\n{}\n\n\
            评分标准:\n{}\n\n\
            技术要求:\n{}\n\n\
            安全规则:\n{}\n",
            self.hill_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_criteria().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.technique().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ski_jumping_rules() {
        let rules = SkiJumpingRules::new();
        assert!(!rules.hill_types().is_empty());
    }
}