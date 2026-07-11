//! 刚柔流空手道规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 刚柔流空手道规则
pub struct GojuRyuRules {
    metadata: RuleMetadata,
}

impl GojuRyuRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("刚柔流空手道规则", "刚柔流空手道比赛基本规则")
                .with_origin("日本冲绳")
                .with_tags(vec!["体育".into(), "武术".into(), "空手道".into()]),
        }
    }

    /// 刚柔流特点
    pub fn characteristics(&self) -> Vec<&'static str> {
        vec![
            "刚法: 硬性技术，力量打击",
            "柔法: 柔性技术，流畅动作",
            "刚柔一体: 硬柔结合",
            "呼吸法: 呼吸控制",
            "近距离技: 短距离格斗",
        ]
    }

    /// 基本套路
    pub fn kata(&self) -> Vec<&'static str> {
        vec![
            "击碎第一: 基础刚法套路",
            "击碎第二: 进阶刚法套路",
            "碎破: 柔法套路",
            "制引战: 传统套路",
            "十三: 高级套路",
            "一百零八: 最高级套路",
        ]
    }

    /// 训练方法
    pub fn training_methods(&self) -> Vec<&'static str> {
        vec![
            "基本练习: 动作训练",
            "移动练习: 步法训练",
            "组手练习: 对抗训练",
            "套路练习: 型训练",
            "呼吸练习: 呼吸法",
            "辅助训练: 器械辅助",
        ]
    }

    /// 基本技法
    pub fn basic_techniques(&self) -> Vec<&'static str> {
        vec![
            "突技: 拳法攻击",
            "受技: 防守技术",
            "蹴技: 腿法攻击",
            "投技: 摔投技术",
            "关节技: 关节控制",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "套路比赛: 型表演评分",
            "组手比赛: 对抗比赛",
            "团体比赛: 团队竞赛",
            "时间限制: 套路和组手时限",
            "裁判评分制",
        ]
    }

    /// 得分标准
    pub fn scoring_criteria(&self) -> Vec<&'static str> {
        vec![
            "一本: 完美技术得分",
            "技有: 有效技术得分",
            "有效: 基本效果得分",
            "套路评分: 动作规范性",
            "判定胜: 平局评分",
        ]
    }

    /// 禁止行为
    pub fn prohibited_actions(&self) -> Vec<&'static str> {
        vec![
            "过度接触打击",
            "攻击危险部位",
            "危险投技",
            "消极比赛",
            "不尊重裁判",
        ]
    }

    /// 辅助训练
    pub fn supplementary_training(&self) -> Vec<&'static str> {
        vec![
            "卷藁: 拳击训练板",
            "亚铃: 传统哑铃",
            "握力器: 握力训练",
            "铁锁: 力量训练",
            "杠铃: 现代器械",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "佩戴护具: 组手必备",
            "控制打击: 安全接触",
            "赛前体检: 健康检查",
            "医疗支持: 赛场保障",
            "禁止危险动作",
        ]
    }
}

impl Default for GojuRyuRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for GojuRyuRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("goju_ryu")
    }

    fn explain(&self) -> String {
        format!(
            "【刚柔流空手道规则】\n\n\
            刚柔流特点:\n{}\n\n\
            基本套路:\n{}\n\n\
            基本技法:\n{}\n\n\
            比赛规则:\n{}\n\n\
            安全规则:\n{}\n",
            self.characteristics()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.kata()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.basic_techniques()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.competition_rules()
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
    fn test_goju_ryu_rules() {
        let rules = GojuRyuRules::new();
        assert!(!rules.characteristics().is_empty());
        assert!(!rules.kata().is_empty());
        assert!(!rules.basic_techniques().is_empty());
    }

    #[test]
    fn test_goju_ryu_kata() {
        let rules = GojuRyuRules::new();
        let katas = rules.kata();
        assert!(katas.contains(&"击碎第一: 基础刚法套路"));
        assert!(katas.contains(&"一百零八: 最高级套路"));
    }
}