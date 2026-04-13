//! 拉丁舞规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 拉丁舞规则
pub struct LatinDanceRules {
    metadata: RuleMetadata,
}

impl LatinDanceRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "拉丁舞规则",
                "拉丁舞比赛规则"
            )
            .with_origin("拉丁美洲")
            .with_tags(vec!["体育".into(), "舞蹈".into()]),
        }
    }

    /// 比赛项目
    pub fn dance_types(&self) -> Vec<&'static str> {
        vec![
            "桑巴",
            "恰恰恰",
            "伦巴",
            "帕索多ble",
            "牛仔舞",
        ]
    }

    /// 技术要素
    pub fn technical_elements(&self) -> Vec<&'static str> {
        vec![
            "节奏控制",
            "臀部动作",
            "脚步技术",
            "身体线条",
            "搭档配合",
        ]
    }

    /// 评分标准
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "技术质量",
            "音乐节奏",
            "表演能力",
            "身体表达",
            "整体表现",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛轮次",
            "服装规定",
            "音乐规则",
            "场地要求",
            "裁判评分",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "比赛服装",
            "舞鞋",
            "音乐选择",
            "场地装备",
            "附属配件",
        ]
    }

    /// 级别体系
    pub fn skill_levels(&self) -> Vec<&'static str> {
        vec![
            "初级级别",
            "中级水平",
            "高级水平",
            "专业级别",
            "教练认证",
        ]
    }

    /// 音乐规则
    pub fn music_rules(&self) -> Vec<&'static str> {
        vec![
            "节奏要求",
            "音乐选择",
            "时间限制",
            "音乐配合",
            "音乐表达",
        ]
    }
}

impl Default for LatinDanceRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for LatinDanceRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("latin_dance")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【拉丁舞规则】\n\n\
            比赛项目:\n{}\n\n\
            技术要素:\n{}\n\n\
            评分标准:\n{}\n\n\
            装备要求:\n{}\n",
            self.dance_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.technical_elements().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_latin_dance_rules() {
        let rules = LatinDanceRules::new();
        assert!(!rules.dance_types().is_empty());
    }
}