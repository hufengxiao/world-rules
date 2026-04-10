//! 垒球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 垒球规则
pub struct SoftballRules {
    metadata: RuleMetadata,
}

impl SoftballRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "垒球规则",
                "垒球比赛基本规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "球类".into()]),
        }
    }

    /// 比赛类型
    pub fn game_types(&self) -> Vec<&'static str> {
        vec![
            "快投垒球: 竞技比赛",
            "慢投垒球: 娱乐比赛",
            "女子垒球: 奥运会项目",
            "7局制比赛",
            "平局延长赛",
        ]
    }

    /// 场地规格
    pub fn field_dimensions(&self) -> Vec<&'static str> {
        vec![
            "垒间距离: 18.29米(快投)",
            "投手板距本垒: 13.11米(女子)",
            "本垒板: 五角形",
            "外场: 扇形区域",
            "内场: 正方形",
        ]
    }

    /// 队员配置
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "场上队员: 9人(快投)",
            "10人(慢投)",
            "替补队员: 可随时换人",
            "指定打击: 代替投手打击",
            "替补规则与棒球类似",
        ]
    }

    /// 比赛局数
    pub fn innings(&self) -> Vec<&'static str> {
        vec![
            "标准比赛: 7局",
            "每局分上下半局",
            "三出局交换攻守",
            "平局时延长赛",
            "国际比赛有封局规则",
        ]
    }

    /// 投球规则
    pub fn pitching_rules(&self) -> Vec<&'static str> {
        vec![
            "快投: 可快速投球",
            "慢投: 球必须有弧度",
            "投球必须下手投掷",
            "好球带: 膝盖到胸部",
            "四坏球保送一垒",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "跑者依次踏过各垒回到本垒",
            "全垒打: 球飞出外场",
            "安打: 击球后上垒",
            "保送: 四坏球或触身球",
            "得分多者获胜",
        ]
    }

    /// 出局方式
    pub fn out_methods(&self) -> Vec<&'static str> {
        vec![
            "三振: 三个好球",
            "接杀: 击出球被接住",
            "封杀: 球传到垒位",
            "触杀: 用球触碰跑者",
            "界外球接杀",
        ]
    }
}

impl Default for SoftballRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SoftballRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("softball")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【垒球规则】\n\n\
            比赛类型:\n{}\n\n\
            投球规则:\n{}\n\n\
            得分规则:\n{}\n\n\
            出局方式:\n{}\n",
            self.game_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.pitching_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.out_methods().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_softball_rules() {
        let rules = SoftballRules::new();
        assert!(!rules.game_types().is_empty());
    }
}