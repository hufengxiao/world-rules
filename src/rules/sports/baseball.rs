//! 棒球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 棒球规则
pub struct BaseballRules {
    metadata: RuleMetadata,
}

impl BaseballRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "棒球规则",
                "棒球比赛基本规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "球类".into()]),
        }
    }

    /// 场地布局
    pub fn field_layout(&self) -> Vec<&'static str> {
        vec![
            "内场: 正方形，边长约27米",
            "四个垒位: 一垒、二垒、三垒、本垒",
            "外场: 内场以外的扇形区域",
            "投手丘: 位于内场中心",
            "击球区: 本垒两侧",
        ]
    }

    /// 队员配置
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "防守方: 9名场上队员",
            "投手、捕手、四个内场手、三个外场手",
            "进攻方: 9名击球员轮流击球",
            "替补队员: 可随时换人",
        ]
    }

    /// 比赛局数
    pub fn innings(&self) -> Vec<&'static str> {
        vec![
            "标准比赛: 9局",
            "每局分上下半局",
            "上半局客队进攻",
            "下半局主队进攻",
            "平局时延长赛",
            "职业延长赛最多12局",
        ]
    }

    /// 出局方式
    pub fn out_methods(&self) -> Vec<&'static str> {
        vec![
            "三振: 击球员三次挥棒失败",
            "接杀: 击出球被防守方直接接住",
            "封杀: 球传到垒位先于跑者到达",
            "触杀: 防守队员用球触碰跑者",
            "界外球接杀",
        ]
    }

    /// 得分方式
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "跑者依次经过一、二、三垒回到本垒得1分",
            "全垒打: 击球员直接绕场一圈",
            "满垒全垒打得4分(大满贯)",
            "失误得分: 防守失误导致得分",
        ]
    }

    /// 好球带
    pub fn strike_zone(&self) -> Vec<&'static str> {
        vec![
            "高度: 击球员膝盖到胸部之间",
            "宽度: 本垒板的宽度",
            "好球: 球穿过好球带",
            "坏球: 球未穿过好球带",
            "击球员挥棒无论位置都是好球",
        ]
    }

    /// 常见术语
    pub fn terminology(&self) -> Vec<&'static str> {
        vec![
            "安打: 击球员成功上垒",
            "全垒打: 击球飞出外场围墙",
            "盗垒: 跑者在投球时前进",
            "保送: 四坏球后击球员免费上垒",
            "双杀: 一次防守制造两个出局",
            "三杀: 一次防守制造三个出局",
            "牺牲打: 为让队友推进而出局",
        ]
    }

    /// 比赛结束条件
    pub fn game_end(&self) -> Vec<&'static str> {
        vec![
            "9局结束后得分高者获胜",
            "主队下半局领先时无需完成",
            "提前结束: 双方差距过大",
            "雨天中止规则",
        ]
    }
}

impl Default for BaseballRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BaseballRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("baseball")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【棒球规则】\n\n\
            场地布局:\n{}\n\n\
            出局方式:\n{}\n\n\
            得分方式:\n{}\n\n\
            常见术语:\n{}\n",
            self.field_layout().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.out_methods().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.terminology().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_baseball_rules() {
        let rules = BaseballRules::new();
        assert!(!rules.field_layout().is_empty());
        assert!(!rules.out_methods().is_empty());
    }
}