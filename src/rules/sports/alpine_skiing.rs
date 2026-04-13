//! 高山滑雪规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 高山滑雪规则
pub struct AlpineSkiingRules {
    metadata: RuleMetadata,
}

impl AlpineSkiingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "高山滑雪规则",
                "高山滑雪比赛基本规则"
            )
            .with_origin("奥地利")
            .with_tags(vec!["体育".into(), "冬季".into()]),
        }
    }

    /// 比赛项目
    pub fn competition_events(&self) -> Vec<&'static str> {
        vec![
            "滑降: 高速下坡",
            "超级大回转: SG",
            "大回转: GS",
            "回转: SL",
            "全能: 滑降+回转",
        ]
    }

    /// 滑降规则
    pub fn downhill_rules(&self) -> Vec<&'static str> {
        vec![
            "最高速度项目",
            "赛道落差800米以上",
            "一次性滑行",
            "旗门设置较少",
            "速度可达130公里/小时",
        ]
    }

    /// 回转规则
    pub fn slalom_rules(&self) -> Vec<&'static str> {
        vec![
            "最技术性项目",
            "旗门设置密集",
            "两轮滑行",
            "旗门间距15米以下",
            "转弯频繁",
        ]
    }

    /// 大回转规则
    pub fn giant_slalom_rules(&self) -> Vec<&'static str> {
        vec![
            "中等技术难度",
            "旗门间距较大",
            "两轮滑行",
            "弯道较宽",
            "速度适中",
        ]
    }

    /// 旗门规则
    pub fn gate_rules(&self) -> Vec<&'static str> {
        vec![
            "红蓝旗门交替",
            "必须通过旗门",
            "两杆之间穿过",
            "漏门取消资格",
            "旗门间隔规则",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "滑雪板: 不同项目不同长度",
            "滑雪靴: 硬壳靴",
            "固定器: DIN值调整",
            "头盔: 必须佩戴",
            "护具: 保护装备",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "头盔必须佩戴",
            "赛道安全检查",
            "天气条件监控",
            "医疗支持",
            "安全网设置",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "漏门",
            "旗门错误通过",
            "摔倒后重新通过",
            "装备违规",
            "超时",
        ]
    }
}

impl Default for AlpineSkiingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AlpineSkiingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("alpine_skiing")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【高山滑雪规则】\n\n\
            比赛项目:\n{}\n\n\
            旗门规则:\n{}\n\n\
            安全规则:\n{}\n\n\
            犯规规则:\n{}\n",
            self.competition_events().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.gate_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alpine_skiing_rules() {
        let rules = AlpineSkiingRules::new();
        assert!(!rules.competition_events().is_empty());
    }
}