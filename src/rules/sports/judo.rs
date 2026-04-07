//! 柔道规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 柔道规则
pub struct JudoRules {
    metadata: RuleMetadata,
}

impl JudoRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "柔道规则",
                "柔道比赛基本规则"
            )
            .with_origin("日本")
            .with_tags(vec!["体育".into(), "格斗".into()]),
        }
    }

    /// 体重级别
    pub fn weight_classes(&self) -> Vec<&'static str> {
        vec![
            "男子: -60kg, -66kg, -73kg, -81kg, -90kg, -100kg, +100kg",
            "女子: -48kg, -52kg, -57kg, -63kg, -70kg, -78kg, +78kg",
        ]
    }

    /// 技术等级
    pub fn technique_levels(&self) -> Vec<&'static str> {
        vec![
            "一本(Ippon): 完美技术，直接获胜",
            "技有(Waza-ari): 接近完美的技术",
            "有效(Yuko): 有效技术",
            "效果(Koka): 基本效果(已取消)",
            "两个技有等于一本",
        ]
    }

    /// 一本条件
    pub fn ippon_conditions(&self) -> Vec<&'static str> {
        vec![
            "投技: 将对手大幅度摔倒落地",
            "固技: 压制对手20秒以上",
            "对手投降",
            "对手昏迷",
            "对手被判犯规出局",
        ]
    }

    /// 比赛时间
    pub fn match_duration(&self) -> Vec<&'static str> {
        vec![
            "男子: 5分钟(正赛)",
            "女子: 4分钟(正赛)",
            "青少年比赛时间较短",
            "金分加时: 无时间限制",
        ]
    }

    /// 禁止行为
    pub fn prohibited_actions(&self) -> Vec<&'static str> {
        vec![
            "故意伤害对手",
            "攻击非柔道技术部位",
            "用拳头或脚攻击",
            "抓对手头发或脸部",
            "用腿夹住对手颈部",
            "故意拖延比赛",
            "假装受伤",
        ]
    }

    /// 柔道服要求
    pub fn uniform_requirements(&self) -> Vec<&'static str> {
        vec![
            "上衣长度: 遮盖大腿中部",
            "袖子长度: 超过手腕5厘米",
            "裤子长度: 遮盖小腿大部分",
            "腰带: 绑紧上衣",
            "颜色: 白色或蓝色",
        ]
    }

    /// 场地要求
    pub fn competition_area(&self) -> Vec<&'static str> {
        vec![
            "比赛区: 最小8x8米",
            "安全区: 至少2米宽度",
            "场地颜色区分",
            "榻榻米材质",
        ]
    }
}

impl Default for JudoRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for JudoRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("judo")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【柔道规则】\n\n\
            技术等级:\n{}\n\n\
            一本条件:\n{}\n\n\
            比赛时间:\n{}\n\n\
            禁止行为:\n{}\n",
            self.technique_levels().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.ippon_conditions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.match_duration().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.prohibited_actions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_judo_rules() {
        let rules = JudoRules::new();
        assert!(!rules.weight_classes().is_empty());
        assert!(!rules.ippon_conditions().is_empty());
    }
}