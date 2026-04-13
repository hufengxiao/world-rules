//! 合气道规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 合气道规则
pub struct AikidoRules {
    metadata: RuleMetadata,
}

impl AikidoRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "合气道规则",
                "合气道比赛基本规则"
            )
            .with_origin("日本")
            .with_tags(vec!["体育".into(), "武术".into()]),
        }
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "演武表演",
            "技法展示",
            "比赛较少",
            "级别评定",
            "团体演武",
        ]
    }

    /// 技术类别
    pub fn technique_categories(&self) -> Vec<&'static str> {
        vec![
            "投技: 投摔技术",
            "固技: 控制技术",
            "当身技: 打击技术",
            "武器技: 杖技术",
            "呼吸投: 无触投摔",
        ]
    }

    /// 基本技法
    pub fn basic_techniques(&self) -> Vec<&'static str> {
        vec![
            "四方投",
            "入身投",
            "小手返",
            "肘压",
            "天地投",
        ]
    }

    /// 级位制度
    pub fn ranking_system(&self) -> Vec<&'static str> {
        vec![
            "初段至十段",
            "级别评定",
            "演武考核",
            "技术要求",
            "年限规定",
        ]
    }

    /// 训练服装
    pub fn training_uniform(&self) -> Vec<&'static str> {
        vec![
            "道服: 上衣和裤子",
            "腰带: 级别标识",
            "袴: 高级别穿",
            "颜色规定",
            "整洁要求",
        ]
    }

    /// 场地要求
    pub fn dojo_requirements(&self) -> Vec<&'static str> {
        vec![
            "道场铺设榻榻米",
            "面积足够",
            "通风良好",
            "祭坛设置",
            "安全设施",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "控制力度",
            "配合练习",
            "避免受伤",
            "医疗支持",
            "循序渐进",
        ]
    }

    /// 传统礼仪
    pub fn traditions(&self) -> Vec<&'static str> {
        vec![
            "向神坛行礼",
            "师生行礼",
            "练习前后礼",
            "服装整洁",
            "精神修养",
        ]
    }
}

impl Default for AikidoRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AikidoRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("aikido")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【合气道规则】\n\n\
            技术类别:\n{}\n\n\
            基本技法:\n{}\n\n\
            级位制度:\n{}\n\n\
            安全规则:\n{}\n",
            self.technique_categories().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.basic_techniques().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.ranking_system().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aikido_rules() {
        let rules = AikidoRules::new();
        assert!(!rules.technique_categories().is_empty());
    }
}