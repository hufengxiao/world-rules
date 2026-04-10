//! 巴西柔术规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 巴西柔术规则
pub struct BjjRules {
    metadata: RuleMetadata,
}

impl BjjRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "巴西柔术规则",
                "巴西柔术比赛基本规则"
            )
            .with_origin("巴西")
            .with_tags(vec!["体育".into(), "格斗".into()]),
        }
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "道服比赛(Gi): 穿着道服",
            "无道服比赛: 穿着紧身衣",
            "积分制比赛",
            "淘汰赛和循环赛",
            "分级比赛按体重和技术",
        ]
    }

    /// 体重级别
    pub fn weight_classes(&self) -> Vec<&'static str> {
        vec![
            "男性: 从超轻量到超重量",
            "女性: 从轻量到重量级",
            "体重间隔约5-10公斤",
            "赛前称重",
            "超重需调整级别",
        ]
    }

    /// 技术等级
    pub fn belt_levels(&self) -> Vec<&'static str> {
        vec![
            "白带: 初学者",
            "蓝带: 2年以上经验",
            "紫带: 4年以上经验",
            "棕带: 6年以上经验",
            "黑带: 8年以上经验",
        ]
    }

    /// 得分系统
    pub fn scoring_system(&self) -> Vec<&'static str> {
        vec![
            "背位控制: 4分",
            "骑乘位: 4分",
            "侧控: 3分",
            "守卫位过腿: 3分",
            "扫技: 2分",
            "膝骑乘位: 2分",
        ]
    }

    /// 优势判定
    pub fn advantages(&self) -> Vec<&'static str> {
        vec![
            "接近得分动作",
            "进攻主动性",
            "有效压制时间",
            "技术执行接近完成",
            "用于积分相同时判定",
        ]
    }

    /// 降服技术
    pub fn submissions(&self) -> Vec<&'static str> {
        vec![
            "绞技: 颈部压迫",
            "关节技: 手臂和腿部",
            "肌肉压迫: 肌肉群锁",
            "拍地认输",
            " verbally认输",
        ]
    }

    /// 禁止动作
    pub fn prohibited_actions(&self) -> Vec<&'static str> {
        vec![
            "攻击脊椎和颈部扭曲",
            "手指扭转",
            "抓挠和咬人",
            "白带禁止部分关节技",
            "18岁以下限制更多",
        ]
    }

    /// 比赛时间
    pub fn match_duration(&self) -> Vec<&'static str> {
        vec![
            "白带: 4-5分钟",
            "蓝带: 5-6分钟",
            "紫带: 6-7分钟",
            "棕带: 7-8分钟",
            "黑带: 8-10分钟",
        ]
    }
}

impl Default for BjjRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BjjRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("bjj")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【巴西柔术规则】\n\n\
            技术等级:\n{}\n\n\
            得分系统:\n{}\n\n\
            降服技术:\n{}\n\n\
            比赛时间:\n{}\n",
            self.belt_levels().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_system().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.submissions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.match_duration().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bjj_rules() {
        let rules = BjjRules::new();
        assert!(!rules.scoring_system().is_empty());
    }
}