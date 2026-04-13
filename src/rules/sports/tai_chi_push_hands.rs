//! 太极推手规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 太极推手规则
pub struct TaiChiPushHandsRules {
    metadata: RuleMetadata,
}

impl TaiChiPushHandsRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "太极推手规则",
                "太极推手比赛基本规则"
            )
            .with_origin("中国")
            .with_tags(vec!["体育".into(), "武术".into()]),
        }
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "定步推手",
            "活步推手",
            "散手推手",
            "个人比赛",
            "团体比赛",
        ]
    }

    /// 技术要求
    pub fn technique_requirements(&self) -> Vec<&'static str> {
        vec![
            "粘连黏随",
            "不丢不顶",
            "引进落空",
            "借力打力",
            "以柔克刚",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "两人对决",
            "推手时间限制",
            "得分判定",
            "犯规处罚",
            "胜负判定",
        ]
    }

    /// 得分标准
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "推出对方得分",
            "迫使对方移动",
            "技术优势",
            "控制能力",
            "太极原则运用",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "使用硬力",
            "抓握对方",
            "攻击动作",
            "不当行为",
            "超出比赛区域",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "太极服装",
            "软底鞋",
            "比赛场地平整",
            "无器械",
            "号码标识",
        ]
    }

    /// 场地规格
    pub fn field_specifications(&self) -> Vec<&'static str> {
        vec![
            "定步圆圈: 直径3米",
            "活步方场: 6×6米",
            "地面平整",
            "边界清晰",
            "裁判位置",
        ]
    }

    /// 传统原则
    pub fn traditional_principles(&self) -> Vec<&'static str> {
        vec![
            "听劲懂劲",
            "虚实分明",
            "内外合一",
            "气沉丹田",
            "舍己从人",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "控制力度",
            "避免伤害",
            "医疗支持",
            "裁判监督",
            "选手保护",
        ]
    }
}

impl Default for TaiChiPushHandsRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for TaiChiPushHandsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("tai_chi_push_hands")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【太极推手规则】\n\n\
            技术要求:\n{}\n\n\
            得分标准:\n{}\n\n\
            犯规规则:\n{}\n\n\
            传统原则:\n{}\n",
            self.technique_requirements().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.traditional_principles().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tai_chi_push_hands_rules() {
        let rules = TaiChiPushHandsRules::new();
        assert!(!rules.technique_requirements().is_empty());
    }
}