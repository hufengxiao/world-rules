//! 极真会馆空手道规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 极真会馆空手道规则
pub struct KyokushinRules {
    metadata: RuleMetadata,
}

impl KyokushinRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("极真会馆空手道规则", "极真会馆空手道比赛基本规则")
                .with_origin("日本")
                .with_tags(vec!["体育".into(), "武术".into(), "空手道".into()]),
        }
    }

    /// 比赛特点
    pub fn competition_characteristics(&self) -> Vec<&'static str> {
        vec![
            "全接触: 实际打击",
            "无护具: 直接对抗",
            "实战性: 真实技击",
            "精神力: 忍耐考验",
            "硬派风格: 强硬打法",
        ]
    }

    /// 允许技术
    pub fn permitted_techniques(&self) -> Vec<&'static str> {
        vec![
            "拳法: 直拳、勾拳",
            "腿法: 低段、中段踢",
            "膝击: 膝盖攻击",
            "肘击: 手肘攻击",
            "格挡: 防守技术",
        ]
    }

    /// 禁止技术
    pub fn prohibited_techniques(&self) -> Vec<&'static str> {
        vec![
            "头部打击: 手部攻击头部",
            "喉部攻击: 喉咙部位",
            "后脑攻击: 后脑勺",
            "脊椎攻击: 背部脊椎",
            "关节攻击: 关节破坏",
            "地面攻击: 已倒地对手",
        ]
    }

    /// 得分标准
    pub fn scoring_criteria(&self) -> Vec<&'static str> {
        vec![
            "一本: 完美技术击倒",
            "技有: 有效技术得分",
            "有效: 中等效果得分",
            "判定: 比赛结束时裁判评分",
            "延长时间: 平局加时赛",
        ]
    }

    /// 比赛时间
    pub fn match_duration(&self) -> Vec<&'static str> {
        vec![
            "正赛: 3分钟",
            "决赛: 4分钟",
            "加时赛: 2分钟",
            "青少年: 时间缩短",
            "休息时间: 局间1分钟",
        ]
    }

    /// 体重级别
    pub fn weight_classes(&self) -> Vec<&'static str> {
        vec![
            "男子轻量级: -65kg",
            "男子中量级: -75kg",
            "男子重量级: -85kg",
            "男子超重量级: +85kg",
            "女子级别: 对应调整",
        ]
    }

    /// 基本套路
    pub fn kata(&self) -> Vec<&'static str> {
        vec![
            "太极: 基础套路系列",
            "平安: 进阶套路系列",
            "拆手: 高级套路",
            "最破: 极真特色套路",
            "征开: 最高级套路",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "赛前体检: 健康检查",
            "医疗支持: 赛场医生",
            "裁判监督: 安全保障",
            "重伤终止: 安全优先",
            "保险要求: 比赛保险",
        ]
    }
}

impl Default for KyokushinRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for KyokushinRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("kyokushin")
    }

    fn explain(&self) -> String {
        format!(
            "【极真会馆空手道规则】\n\n\
            比赛特点:\n{}\n\n\
            允许技术:\n{}\n\n\
            禁止技术:\n{}\n\n\
            得分标准:\n{}\n\n\
            安全规则:\n{}\n",
            self.competition_characteristics()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.permitted_techniques()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.prohibited_techniques()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.scoring_criteria()
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
    fn test_kyokushin_rules() {
        let rules = KyokushinRules::new();
        assert!(!rules.competition_characteristics().is_empty());
        assert!(!rules.permitted_techniques().is_empty());
        assert!(!rules.prohibited_techniques().is_empty());
    }

    #[test]
    fn test_kyokushin_kata() {
        let rules = KyokushinRules::new();
        let katas = rules.kata();
        assert!(katas.contains(&"太极: 基础套路系列"));
        assert!(katas.contains(&"平安: 进阶套路系列"));
    }
}
