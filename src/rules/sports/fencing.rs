//! 击剑规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 击剑规则
pub struct FencingRules {
    metadata: RuleMetadata,
}

impl FencingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "击剑规则",
                "击剑比赛基本规则"
            )
            .with_origin("欧洲")
            .with_tags(vec!["体育".into(), "格斗".into()]),
        }
    }

    /// 剑种类型
    pub fn weapon_types(&self) -> Vec<&'static str> {
        vec![
            "花剑: 轻剑，只能刺",
            "重剑: 全身有效，只能刺",
            "佩剑: 可刺可劈",
            "花剑和佩剑有优先权规则",
            "重剑无优先权规则",
        ]
    }

    /// 有效部位
    pub fn target_areas(&self) -> Vec<&'static str> {
        vec![
            "花剑: 躯干(背心覆盖区域)",
            "重剑: 全身有效",
            "佩剑: 腰部以上(包括头部和手臂)",
            "击中有效部位得分",
            "电子裁判系统判定",
        ]
    }

    /// 比赛形式
    pub fn match_formats(&self) -> Vec<&'static str> {
        vec![
            "个人赛: 15分制，三局每局3分钟",
            "团体赛: 每队3人，接力累计45分",
            "淘汰赛制",
            "平局时一分钟优先权决胜负",
            "世界杯和奥运会项目",
        ]
    }

    /// 剑道规格
    pub fn piste_dimensions(&self) -> Vec<&'static str> {
        vec![
            "剑道: 长14米，宽1.5-2米",
            "中线: 剑道中央",
            "警告线: 距端线2米",
            "端线: 剑道两端",
            "越出端线失一分",
        ]
    }

    /// 优先权规则
    pub fn right_of_way(&self) -> Vec<&'static str> {
        vec![
            "花剑和佩剑适用",
            "进攻方有优先权",
            "防守成功后获得优先权",
            "同时击中时优先权方得分",
            "重剑同时击中双方各得一分",
        ]
    }

    /// 犯规与处罚
    pub fn fouls_penalties(&self) -> Vec<&'static str> {
        vec![
            "黄牌: 警告",
            "红牌: 罚一分",
            "黑牌: 驱逐出场",
            "危险动作处罚",
            "不公平比赛处罚",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "面罩: 保护头部",
            "护胸: 保护躯干",
            "手套: 保护持剑手",
            "剑服: 白色防护服",
            "金属背心(花剑佩剑)",
        ]
    }
}

impl Default for FencingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FencingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("fencing")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【击剑规则】\n\n\
            剑种类型:\n{}\n\n\
            有效部位:\n{}\n\n\
            比赛形式:\n{}\n\n\
            优先权规则:\n{}\n",
            self.weapon_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.target_areas().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.match_formats().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.right_of_way().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fencing_rules() {
        let rules = FencingRules::new();
        assert!(!rules.weapon_types().is_empty());
    }
}