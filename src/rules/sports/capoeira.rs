//! 卡波耶拉规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 卡波耶拉规则 (巴西武术舞蹈)
pub struct CapoeiraRules {
    metadata: RuleMetadata,
}

impl CapoeiraRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "卡波耶拉规则",
                "巴西武术舞蹈规则"
            )
            .with_origin("巴西")
            .with_tags(vec!["体育".into(), "武术".into(), "舞蹈".into()]),
        }
    }

    /// 武术流派
    pub fn styles(&self) -> Vec<&'static str> {
        vec![
            "卡波耶拉地区流派",
            "卡波耶拉安哥拉流派",
            "现代卡波耶拉",
            "传统流派",
            "各地变体",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "基步动作",
            "踢腿动作",
            "闪避动作",
            "地面动作",
            "花式动作",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "霍达比赛",
            "音乐配合",
            "流畅性评分",
            "技巧评分",
            "互动评分",
        ]
    }

    /// 音乐元素
    pub fn music_elements(&self) -> Vec<&'static str> {
        vec![
            "贝林报鼓",
            "潘德罗鼓",
            "阿塔巴克鼓",
            "歌唱节奏",
            "音乐指导",
        ]
    }

    /// 得分标准
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "技巧难度",
            "流畅表现",
            "音乐配合",
            "互动能力",
            "创意表达",
        ]
    }

    /// 级别体系
    pub fn cord_system(&self) -> Vec<&'static str> {
        vec![
            "学员绳带",
            "进阶绳带",
            "教练绳带",
            "大师绳带",
            "级别认证",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "比赛服装",
            "绳带标识",
            "乐器装备",
            "比赛场地",
            "安全保护",
        ]
    }
}

impl Default for CapoeiraRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CapoeiraRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("capoeira")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【卡波耶拉规则】\n\n\
            武术流派:\n{}\n\n\
            技术动作:\n{}\n\n\
            音乐元素:\n{}\n\n\
            级别体系:\n{}\n",
            self.styles().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.techniques().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.music_elements().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.cord_system().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capoeira_rules() {
        let rules = CapoeiraRules::new();
        assert!(!rules.styles().is_empty());
    }
}