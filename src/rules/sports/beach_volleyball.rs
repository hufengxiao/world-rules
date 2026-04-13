//! 沙滩排球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 沙滩排球规则
pub struct BeachVolleyballRules {
    metadata: RuleMetadata,
}

impl BeachVolleyballRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "沙滩排球规则",
                "沙滩排球比赛基本规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "排球".into()]),
        }
    }

    /// 球场规格
    pub fn court_specifications(&self) -> Vec<&'static str> {
        vec![
            "场地尺寸: 16×8米",
            "沙滩地面",
            "网高: 2.43米(男子)",
            "网高: 2.24米(女子)",
            "无进攻线",
        ]
    }

    /// 队伍构成
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "每队2人",
            "无替补",
            "分工明确",
            "全面技术",
            "轮转规则",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "三局两胜",
            "前两局21分",
            "决胜局15分",
            "至少领先2分",
            "每球得分制",
        ]
    }

    /// 技术规则
    pub fn technique_rules(&self) -> Vec<&'static str> {
        vec![
            "三人触球上限",
            "不允许双手传球",
            "托球规则",
            "扣球规则",
            "拦网计为触球",
        ]
    }

    /// 发球规则
    pub fn serving_rules(&self) -> Vec<&'static str> {
        vec![
            "轮换发球",
            "发球区规定",
            "允许跳发",
            "8秒内发球",
            "发球得分制",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "四次触球",
            "双手传球",
            "触网",
            "过网犯规",
            "位置犯规",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "沙滩排球",
            "泳装或运动服",
            "无鞋比赛",
            "太阳镜可选",
            "帽子可选",
        ]
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "奥运会项目",
            "世界锦标赛",
            "世界巡回赛",
            "国内比赛",
            "业余比赛",
        ]
    }
}

impl Default for BeachVolleyballRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BeachVolleyballRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("beach_volleyball")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【沙滩排球规则】\n\n\
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
    fn test_beach_volleyball_rules() {
        let rules = BeachVolleyballRules::new();
        assert!(!rules.court_specifications().is_empty());
    }
}