//! 高尔夫规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 高尔夫规则
pub struct GolfRules {
    metadata: RuleMetadata,
}

impl GolfRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "高尔夫规则",
                "R&A/USGA 高尔夫标准规则"
            )
            .with_origin("苏格兰")
            .with_tags(vec!["体育".into(), "高尔夫".into()]),
        }
    }

    /// 标准球场洞数
    pub fn standard_holes(&self) -> u8 {
        18
    }

    /// 标准杆数范围
    pub fn par_range(&self) -> (u8, u8) {
        (3, 5) // 每洞标准杆: 3杆、4杆或5杆
    }

    /// 球场规格
    pub fn course_specifications(&self) -> Vec<&'static str> {
        vec![
            "标准球场: 18洞",
            "总长度: 6000-7000码",
            "球洞直径: 4.25英寸 (10.8厘米)",
            "球洞深度: 至少4英寸",
            "果岭: 球洞所在的短草区域",
            "发球台: 每洞起始位置",
        ]
    }

    /// 球杆限制
    pub fn club_limit(&self) -> u8 {
        14 // 最多携带14支球杆
    }

    /// 基本规则
    pub fn basic_rules(&self) -> Vec<&'static str> {
        vec![
            "从发球台开球",
            "每杆击球算一杆",
            "用最少杆数完成18洞",
            "球停在何处就从何处击打",
            "必须把球打入洞中",
        ]
    }

    /// 计分方式
    pub fn scoring_terms(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("一杆进洞", "Hole in One"),
            ("老鹰球", "Eagle (-2)"),
            ("小鸟球", "Birdie (-1)"),
            ("标准杆", "Par (0)"),
            ("柏忌", "Bogey (+1)"),
            ("双柏忌", "Double Bogey (+2)"),
        ]
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "比杆赛 (Stroke Play): 总杆数最少者胜",
            "比洞赛 (Match Play): 每洞胜负累计",
            "四球赛: 两队各两人，取较好成绩",
            "四人两球赛: 两队各两人，轮流击球",
        ]
    }

    /// 处罚规则
    pub fn penalties(&self) -> Vec<&'static str> {
        vec![
            "球出界: 罚一杆，原地重打",
            "球入水: 罚一杆，在指定区域抛球",
            "球丢失: 罚一杆，在原位置附近抛球",
            "移动球: 罚一杆",
            "超过14支球杆: 每洞罚2杆，最多4杆",
        ]
    }

    /// 礼仪规范
    pub fn etiquette(&self) -> Vec<&'static str> {
        vec![
            "保持安静，不打扰他人",
            "及时让后组通过",
            "修复果岭上的球痕",
            "平整沙坑",
            "保持合理打球速度",
            "着装得体",
        ]
    }

    /// 计算差点
    pub fn calculate_handicap(&self, scores: &[u8], course_rating: f32, slope_rating: u16) -> f32 {
        if scores.is_empty() {
            return 0.0;
        }
        let avg_score = scores.iter().sum::<u8>() as f32 / scores.len() as f32;
        let differential = (avg_score - course_rating) * 113.0 / slope_rating as f32;
        differential * 0.96
    }
}

impl Default for GolfRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for GolfRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("golf")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        let scores = self.scoring_terms();
        format!(
            "【高尔夫规则】\n\n\
            标准洞数: {}洞\n\
            球杆限制: {}支\n\n\
            球场规格:\n{}\n\n\
            基本规则:\n{}\n\n\
            计分方式:\n{}\n\n\
            比赛形式:\n{}\n\n\
            处罚规则:\n{}\n\n\
            礼仪规范:\n{}\n",
            self.standard_holes(),
            self.club_limit(),
            self.course_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.basic_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            scores.iter().map(|(cn, en)| format!("  • {} ({})", cn, en)).collect::<Vec<_>>().join("\n"),
            self.competition_formats().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.penalties().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.etiquette().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_golf_rules() {
        let rules = GolfRules::new();
        assert_eq!(rules.standard_holes(), 18);
        assert_eq!(rules.club_limit(), 14);
    }
}