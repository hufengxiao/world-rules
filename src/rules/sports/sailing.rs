//! 帆船规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 帆船规则
pub struct SailingRules {
    metadata: RuleMetadata,
}

impl SailingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "帆船规则",
                "帆船比赛基本规则"
            )
            .with_origin("英国")
            .with_tags(vec!["体育".into(), "水上".into()]),
        }
    }

    /// 船艇类型
    pub fn boat_classes(&self) -> Vec<&'static str> {
        vec![
            "单体船: 传统帆船",
            "双体船: 稳定性高",
            "小艇: 激光级、乐观级",
            "龙骨船: 大型帆船",
            "帆板: 单人操控",
        ]
    }

    /// 奥运级别
    pub fn olympic_classes(&self) -> Vec<&'static str> {
        vec![
            "激光级: 单人单体",
            "激光雷迪尔: 女子单人",
            "470级: 双人单体",
            "49人级: 高速双人",
            "芬兰人级: 重型单人",
            "NACRA17: 混合多体",
        ]
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "场地赛: 绕标航行",
            "航线赛: 距离比赛",
            "对抗赛: 两人对决",
            "团队赛: 团队对抗",
            "环球赛: 远洋比赛",
        ]
    }

    /// 起航规则
    pub fn start_rules(&self) -> Vec<&'static str> {
        vec![
            "起航线: 船和浮标连线",
            "倒计时: 5分钟信号",
            "预警信号: 10分钟",
            "预备信号: 5分钟",
            "起航信号: 开始",
        ]
    }

    /// 绕标规则
    pub fn mark_rounding(&self) -> Vec<&'static str> {
        vec![
            "左舷绕标: 从左侧通过",
            "右舷绕标: 从右侧通过",
            "门标: 可选择方向",
            "障碍标: 绕行规则",
            "终点线: 通过终点",
        ]
    }

    /// 避碰规则
    pub fn collision_rules(&self) -> Vec<&'static str> {
        vec![
            "右舷船有优先权",
            "同舷时内侧船优先",
            "超船时避让",
            " overtaken船避让",
            "相遇时左舷避让右舷",
        ]
    }

    /// 犯规处罚
    pub fn penalties(&self) -> Vec<&'static str> {
        vec![
            "转向处罚: 两圈旋转",
            "罚分: 比赛扣分",
            "抗议程序",
            "仲裁审理",
            "取消资格",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "必须佩戴救生设备",
            "检查天气条件",
            "携带通讯设备",
            "熟悉水域",
            "遵守航行规则",
        ]
    }
}

impl Default for SailingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SailingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("sailing")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【帆船规则】\n\n\
            奥运级别:\n{}\n\n\
            起航规则:\n{}\n\n\
            避碰规则:\n{}\n\n\
            犯规处罚:\n{}\n",
            self.olympic_classes().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.start_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.collision_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.penalties().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sailing_rules() {
        let rules = SailingRules::new();
        assert!(!rules.boat_classes().is_empty());
    }
}