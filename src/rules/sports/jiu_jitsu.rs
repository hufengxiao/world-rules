//! 柔术规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 柔术规则
pub struct JiuJitsuRules {
    metadata: RuleMetadata,
}

impl JiuJitsuRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "柔术规则",
                "柔术比赛基本规则"
            )
            .with_origin("日本")
            .with_tags(vec!["体育".into(), "格斗".into()]),
        }
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "格斗柔术比赛",
            "演武表演",
            "级别评定",
            "团体比赛",
            "国际比赛",
        ]
    }

    /// 技术类别
    pub fn technique_categories(&self) -> Vec<&'static str> {
        vec![
            "投技: 投摔技术",
            "固技: 控制技术",
            "当身技: 打击技术",
            "关节技: 关节控制",
            "窒息技: 颈部控制",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛时间: 5-10分钟",
            "积分制或KO制",
            "投技得分",
            "固技得分",
            "降服获胜",
        ]
    }

    /// 得分标准
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "完美投技: 一本胜利",
            "有效投技: 积分",
            "控制时间: 积分",
            "降服: 直接获胜",
            "综合评分",
        ]
    }

    /// 禁止动作
    pub fn prohibited_actions(&self) -> Vec<&'static str> {
        vec![
            "攻击脊椎",
            "过度关节扭曲",
            "攻击眼睛",
            "咬人抓挠",
            "危险动作",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "道服",
            "腰带级别标识",
            "护具可选",
            "比赛服装规定",
            "安全检查",
        ]
    }

    /// 级位制度
    pub fn ranking_system(&self) -> Vec<&'static str> {
        vec![
            "白带至黑带",
            "级别评定",
            "演武考核",
            "技术要求",
            "精神修养",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "控制力度",
            "降服及时停止",
            "医疗支持",
            "裁判监督",
            "选手保护",
        ]
    }
}

impl Default for JiuJitsuRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for JiuJitsuRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("jiu_jitsu")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【柔术规则】\n\n\
            技术类别:\n{}\n\n\
            得分标准:\n{}\n\n\
            禁止动作:\n{}\n\n\
            安全规则:\n{}\n",
            self.technique_categories().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.prohibited_actions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jiu_jitsu_rules() {
        let rules = JiuJitsuRules::new();
        assert!(!rules.technique_categories().is_empty());
    }
}