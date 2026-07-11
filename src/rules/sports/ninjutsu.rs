//! 忍术规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 忍术规则
pub struct NinjutsuRules {
    metadata: RuleMetadata,
}

impl NinjutsuRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("忍术规则", "忍术比赛与训练基本规则")
                .with_origin("日本")
                .with_tags(vec!["体育".into(), "武术".into(), "忍者".into()]),
        }
    }

    /// 忍术流派
    pub fn schools(&self) -> Vec<&'static str> {
        vec![
            "伊贺流: 伊贺忍者传统",
            "甲贺流: 甲贺忍者传统",
            "户隐流: 户隐忍者传承",
            "九鬼流: 九鬼流派",
            "柳生流: 柳生新阴流",
        ]
    }

    /// 基本技法
    pub fn basic_techniques(&self) -> Vec<&'static str> {
        vec![
            "忍术体术: 格斗技术",
            "剑术: 武士刀技法",
            "手里剑术: 投掷武器",
            "隐身术: 潜行技术",
            "逃脱术: 撤退技术",
        ]
    }

    /// 武器技能
    pub fn weapons(&self) -> Vec<&'static str> {
        vec![
            "忍刀: 短刀技术",
            "手里剑: 投掷飞镖",
            "锁镰: 链镰武器",
            "吹箭: 竹筒暗器",
            "烟雾弹: 辅助工具",
        ]
    }

    /// 潜行技术
    pub fn stealth_techniques(&self) -> Vec<&'static str> {
        vec![
            "隐藏身形: 环境利用",
            "无声移动: 步法训练",
            "观察侦查: 情报获取",
            "伪装变化: 外表隐藏",
            "逃脱撤退: 安全撤离",
        ]
    }

    /// 训练方法
    pub fn training_methods(&self) -> Vec<&'static str> {
        vec![
            "体能训练: 力量和耐力",
            "武器训练: 投掷和使用",
            "潜行训练: 移动和隐藏",
            "格斗训练: 体术技法",
            "精神训练: 冥想和专注",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "套路表演: 技术展示",
            "武器演示: 投掷评分",
            "格斗比赛: 体术对抗",
            "裁判评分制",
            "安全规则优先",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "训练护具: 必备装备",
            "武器钝化: 安全武器",
            "场地要求: 安全环境",
            "医疗支持: 赛场保障",
            "禁止危险动作",
        ]
    }

    /// 现代应用
    pub fn modern_application(&self) -> Vec<&'static str> {
        vec![
            "自卫技术: 安全防卫",
            "体能训练: 综合锻炼",
            "精神修养: 心理建设",
            "表演艺术: 文化展示",
            "武术传承: 传统保留",
        ]
    }
}

impl Default for NinjutsuRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for NinjutsuRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("ninjutsu")
    }

    fn explain(&self) -> String {
        format!(
            "【忍术规则】\n\n\
            忍术流派:\n{}\n\n\
            基本技法:\n{}\n\n\
            武器技能:\n{}\n\n\
            潜行技术:\n{}\n\n\
            安全规则:\n{}\n",
            self.schools()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.basic_techniques()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.weapons()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.stealth_techniques()
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
    fn test_ninjutsu_rules() {
        let rules = NinjutsuRules::new();
        assert!(!rules.schools().is_empty());
        assert!(!rules.basic_techniques().is_empty());
        assert!(!rules.weapons().is_empty());
    }

    #[test]
    fn test_ninjutsu_schools() {
        let rules = NinjutsuRules::new();
        let schools = rules.schools();
        assert!(schools.contains(&"伊贺流: 伊贺忍者传统"));
        assert!(schools.contains(&"甲贺流: 甲贺忍者传统"));
    }
}
