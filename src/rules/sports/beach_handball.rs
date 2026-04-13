//! 沙滩手球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 沙滩手球规则
pub struct BeachHandballRules {
    metadata: RuleMetadata,
}

impl BeachHandballRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "沙滩手球规则",
                "沙滩手球比赛基本规则"
            )
            .with_origin("欧洲")
            .with_tags(vec!["体育".into(), "手球".into()]),
        }
    }

    /// 球场规格
    pub fn court_specifications(&self) -> Vec<&'static str> {
        vec![
            "场地尺寸: 27×12米",
            "沙滩地面",
            "球门区半径6米",
            "门宽: 3米×高2米",
            "无防守线",
        ]
    }

    /// 队伍构成
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "每队4人上场",
            "1名守门员",
            "3名场上球员",
            "替补4人",
            "换人次数不限",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "两半场各10分钟",
            "半场休息5分钟",
            "得分规则特殊",
            "常规得分1分",
            "精彩得分2分",
        ]
    }

    /// 技术规则
    pub fn technique_rules(&self) -> Vec<&'static str> {
        vec![
            "传球技术",
            "射门技术",
            "守门员技术",
            "空中射门加分",
            "360度转身射门加分",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "身体犯规",
            "推搡",
            "阻挡",
            "危险动作",
            "守门员区犯规",
        ]
    }

    /// 罚球规则
    pub fn penalty_rules(&self) -> Vec<&'static str> {
        vec![
            "6米罚球",
            "守门员防守",
            "罚球得分规则",
            "犯规累积",
            "裁判判定",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "沙滩手球",
            "泳装或运动服",
            "无鞋比赛",
            "号码标识",
            "守门员装备",
        ]
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "世界锦标赛",
            "欧洲杯",
            "国内比赛",
            "沙滩运动会",
            "友谊赛",
        ]
    }
}

impl Default for BeachHandballRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BeachHandballRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("beach_handball")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【沙滩手球规则】\n\n\
            球场规格:\n{}\n\n\
            比赛规则:\n{}\n\n\
            技术规则:\n{}\n\n\
            犯规规则:\n{}\n",
            self.court_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.competition_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.technique_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beach_handball_rules() {
        let rules = BeachHandballRules::new();
        assert!(!rules.court_specifications().is_empty());
    }
}