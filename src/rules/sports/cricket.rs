//! 板球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 板球规则
pub struct CricketRules {
    metadata: RuleMetadata,
}

impl CricketRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "板球规则",
                "板球比赛基本规则"
            )
            .with_origin("英国")
            .with_tags(vec!["体育".into(), "球类".into()]),
        }
    }

    /// 比赛形式
    pub fn match_formats(&self) -> Vec<&'static str> {
        vec![
            "测试赛: 5天，两局制",
            "一日国际赛: 50回合，一局制",
            "T20: 20回合，一局制",
            "每回合6个球",
            "每队11名球员",
        ]
    }

    /// 场地布局
    pub fn field_layout(&self) -> Vec<&'static str> {
        vec![
            "球场: 圆形或椭圆形",
            "球道: 中央22码长矩形区域",
            "三柱门: 球道两端各一组",
            "三柱门由三根木桩和两根横木组成",
            "击球区: 三柱门前方区域",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "跑分: 击球后两名击球手交换位置得1分",
            "边界4分: 球滚出边界",
            "边界6分: 球飞出边界未落地",
            "宽球: 守方失误加1分",
            "无球: 守方犯规加1分",
        ]
    }

    /// 出局方式
    pub fn dismissal_types(&self) -> Vec<&'static str> {
        vec![
            "投杀: 球击中三柱门",
            "接杀: 击出球被接住",
            "腿截球: 腿挡住会击中三柱门的球",
            "跑杀: 跑分时三柱门被击中",
            "截杀: 击球手离开击球区被击中三柱门",
            "击球手犯规: 故意干扰比赛",
        ]
    }

    /// 投球规则
    pub fn bowling_rules(&self) -> Vec<&'static str> {
        vec![
            "投球手必须合法投球",
            "手臂伸直超过肩部",
            "不得踩越投球线",
            "每回合6个有效球",
            "宽球和无球不计入回合",
        ]
    }

    /// 击球规则
    pub fn batting_rules(&self) -> Vec<&'static str> {
        vec![
            "两名击球手同时在场上",
            "击球手保护三柱门",
            "击球后可选择跑分",
            "击球手可被判出局",
            "全队10人出局后回合结束",
        ]
    }

    /// 裁判职责
    pub fn umpire_duties(&self) -> Vec<&'static str> {
        vec![
            "两名场上裁判",
            "主裁: 站在投球端",
            "副裁: 站在对方端",
            "第三裁判: 视频回放裁决",
            "裁判决定最终裁决",
        ]
    }
}

impl Default for CricketRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CricketRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("cricket")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【板球规则】\n\n\
            比赛形式:\n{}\n\n\
            得分规则:\n{}\n\n\
            出局方式:\n{}\n\n\
            投球规则:\n{}\n",
            self.match_formats().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.dismissal_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.bowling_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cricket_rules() {
        let rules = CricketRules::new();
        assert!(!rules.match_formats().is_empty());
    }
}