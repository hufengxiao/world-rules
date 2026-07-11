//! 咏春拳规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 咏春拳规则
pub struct WingChunRules {
    metadata: RuleMetadata,
}

impl WingChunRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("咏春拳规则", "咏春拳比赛与训练基本规则")
                .with_origin("中国广东")
                .with_tags(vec!["体育".into(), "武术".into(), "南拳".into()]),
        }
    }

    /// 基本拳法
    pub fn basic_punches(&self) -> Vec<&'static str> {
        vec![
            "日字拳: 直拳攻击中线",
            "摊手: 防守反击手法",
            "膀手: 卸力化解手法",
            "伏手: 控制压制手法",
            "窒手: 突然停顿手法",
        ]
    }

    /// 三套拳法
    pub fn forms(&self) -> Vec<&'static str> {
        vec![
            "小念头: 基础拳法套路",
            "寻桥: 进阶攻防套路",
            "标指: 高级技法套路",
            "木人桩: 器械训练套路",
            "六点半棍: 长棍技法",
        ]
    }

    /// 中线理论
    pub fn centerline_theory(&self) -> Vec<&'static str> {
        vec![
            "守中: 保护自身中线",
            "用中: 攻击对手中线",
            "埋身: 贴身短打",
            "朝面: 正面对敌",
            "追形: 追随对手动作",
        ]
    }

    /// 训练方法
    pub fn training_methods(&self) -> Vec<&'static str> {
        vec![
            "黐手: 双人黏手训练",
            "单黐手: 单手黏手训练",
            "双黐手: 双手黏手训练",
            "木人桩训练: 器械辅助",
            "实战对练: 应用训练",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "套路评分: 动作规范性",
            "黐手比赛: 技术应用",
            "实战比赛: 技击对抗",
            "裁判评分制",
            "体重级别划分",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "佩戴护具: 比赛必备",
            "禁止重击: 控制力度",
            "医疗检查:赛前体检",
            "裁判监督: 安全保障",
            "禁止危险动作",
        ]
    }

    /// 器械要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "木人桩: 标准尺寸",
            "六点半棍: 长度要求",
            "八斩刀: 双刀技法",
            "护具: 比赛必备",
            "训练服: 练习服装",
        ]
    }

    /// 段位制度
    pub fn ranking_system(&self) -> Vec<&'static str> {
        vec![
            "初级: 小念头熟练",
            "中级: 寻桥掌握",
            "高级: 标指精通",
            "教练级: 教学资格",
            "师父级: 完全掌握",
        ]
    }
}

impl Default for WingChunRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for WingChunRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("wing_chun")
    }

    fn explain(&self) -> String {
        format!(
            "【咏春拳规则】\n\n\
            基本拳法:\n{}\n\n\
            三套拳法:\n{}\n\n\
            中线理论:\n{}\n\n\
            训练方法:\n{}\n\n\
            安全规则:\n{}\n",
            self.basic_punches()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.forms()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.centerline_theory()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.training_methods()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.safety_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wing_chun_rules() {
        let rules = WingChunRules::new();
        assert!(!rules.basic_punches().is_empty());
        assert!(!rules.forms().is_empty());
        assert!(!rules.centerline_theory().is_empty());
    }

    #[test]
    fn test_wing_chun_forms() {
        let rules = WingChunRules::new();
        let forms = rules.forms();
        assert!(forms.contains(&"小念头: 基础拳法套路"));
        assert!(forms.contains(&"寻桥: 进阶攻防套路"));
        assert!(forms.contains(&"标指: 高级技法套路"));
    }
}