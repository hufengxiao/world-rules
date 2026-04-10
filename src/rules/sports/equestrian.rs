//! 马术规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 马术规则
pub struct EquestrianRules {
    metadata: RuleMetadata,
}

impl EquestrianRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "马术规则",
                "马术比赛基本规则"
            )
            .with_origin("欧洲")
            .with_tags(vec!["体育".into(), "动物".into()]),
        }
    }

    /// 比赛项目
    pub fn disciplines(&self) -> Vec<&'static str> {
        vec![
            "盛装舞步: 马匹优雅表演",
            "场地障碍: 跨越障碍物",
            "三项赛: 综合3个项目",
            "耐力赛: 长距离比赛",
            "速度赛马: 竞速比赛",
        ]
    }

    /// 盛装舞步规则
    pub fn dressage_rules(&self) -> Vec<&'static str> {
        vec![
            "规定动作序列",
            "评分标准: 0-10分",
            "动作准确性",
            "马匹配合度",
            "优雅流畅度",
        ]
    }

    /// 场地障碍规则
    pub fn show_jumping_rules(&self) -> Vec<&'static str> {
        vec![
            "障碍高度: 最高1.6米",
            "水障宽度: 4.5米",
            "拒跳罚4分",
            "落马罚4分",
            "超时罚分",
            "满分0分",
        ]
    }

    /// 三项赛规则
    pub fn eventing_rules(&self) -> Vec<&'static str> {
        vec![
            "盛装舞步: 第一天",
            "越野赛: 第二天",
            "场地障碍: 第三天",
            "积分累计制",
            "综合评定",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "马鞍: 骑乘支撑",
            "马勒: 控制马匹",
            "马鞭: 适当使用",
            "马刺: 控制辅助",
            "头盔: 安全保护",
        ]
    }

    /// 赛事等级
    pub fn competition_levels(&self) -> Vec<&'static str> {
        vec![
            "国内赛事",
            "国际赛事",
            "奥运会级别",
            "世界锦标赛",
            "分级比赛",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "骑手必须佩戴头盔",
            "马匹健康检查",
            "医疗支持",
            "兽医在场",
            "急救措施",
        ]
    }

    /// 禁止行为
    pub fn prohibited_actions(&self) -> Vec<&'static str> {
        vec![
            "虐待马匹",
            "不当使用马鞭",
            "危险骑乘",
            "药物使用",
            "不当装备",
        ]
    }
}

impl Default for EquestrianRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for EquestrianRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("equestrian")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【马术规则】\n\n\
            比赛项目:\n{}\n\n\
            场地障碍规则:\n{}\n\n\
            装备要求:\n{}\n\n\
            安全规则:\n{}\n",
            self.disciplines().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.show_jumping_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equestrian_rules() {
        let rules = EquestrianRules::new();
        assert!(!rules.disciplines().is_empty());
    }
}