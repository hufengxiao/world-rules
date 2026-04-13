//! BASE跳规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// BASE跳规则
pub struct BaseJumpingRules {
    metadata: RuleMetadata,
}

impl BaseJumpingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "BASE跳规则",
                "BASE跳运动规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "极限".into()]),
        }
    }

    /// BASE含义
    pub fn base_meaning(&self) -> Vec<&'static str> {
        vec![
            "B: Building (建筑)",
            "A: Antenna (天线塔)",
            "S: Span (桥梁)",
            "E: Earth (悬崖)",
            "四种跳伞类型",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "高度限制",
            "装备检查",
            "天气条件",
            "经验要求",
            "地点许可",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "跳跃技术",
            "开伞技术",
            "着陆技术",
            "姿势控制",
            "应急处理",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "BASE降落伞",
            "防护头盔",
            "跳伞服装",
            "高度计",
            "附属装备",
        ]
    }

    /// 经验要求
    pub fn experience_requirements(&self) -> Vec<&'static str> {
        vec![
            "需200次以上跳伞经验",
            "专业培训",
            "认证要求",
            "教练指导",
            "安全课程",
        ]
    }

    /// 法律规则
    pub fn legal_rules(&self) -> Vec<&'static str> {
        vec![
            "地点许可",
            "法律规定",
            "保险要求",
            "责任声明",
            "合规要求",
        ]
    }

    /// 级别体系
    pub fn skill_levels(&self) -> Vec<&'static str> {
        vec![
            "初级BASE跳",
            "中级水平",
            "高级水平",
            "专家级别",
            "教练认证",
        ]
    }
}

impl Default for BaseJumpingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BaseJumpingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("base_jumping")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【BASE跳规则】\n\n\
            BASE含义:\n{}\n\n\
            安全规则:\n{}\n\n\
            装备要求:\n{}\n\n\
            经验要求:\n{}\n",
            self.base_meaning().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.experience_requirements().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_jumping_rules() {
        let rules = BaseJumpingRules::new();
        assert!(!rules.base_meaning().is_empty());
    }
}