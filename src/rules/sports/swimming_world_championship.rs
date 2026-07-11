//! 世界游泳锦标赛规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 世界游泳锦标赛规则
pub struct SwimmingWorldChampionshipRules {
    metadata: RuleMetadata,
}

impl SwimmingWorldChampionshipRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("世界游泳锦标赛规则", "FINA世界游泳锦标赛规则")
                .with_origin("FINA")
                .with_tags(vec!["体育".into(), "游泳".into(), "世锦赛".into()]),
        }
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "自由泳: 50m/100m/200m/400m/800m/1500m",
            "仰泳: 50m/100m/200m",
            "蛙泳: 50m/100m/200m",
            "蝶泳: 50m/100m/200m",
            "个人混合泳: 200m/400m",
            "自由泳接力: 4×100m/4×200m",
            "混合泳接力: 4×100m",
            "男女混合接力: 4×100m自由泳/混合泳",
            "公开水域: 5km/10km/25km",
            "花样游泳: 单人/双人/团体",
        ]
    }

    /// 参赛资格
    pub fn qualification(&self) -> Vec<&'static str> {
        vec![
            "A标: 直接参赛资格",
            "B标: 需国家推荐",
            "每国家每项目最多2人",
            "接力项目: 前12名队伍",
            "外卡: 邀请名额",
            "主办国保送名额",
        ]
    }

    /// 奖金分配
    pub fn prize_money(&self) -> Vec<&'static str> {
        vec![
            "金牌: $20,000",
            "银牌: $15,000",
            "铜牌: $10,000",
            "第4名: $8,000",
            "第5名: $6,000",
            "第6名: $5,000",
            "第7名: $4,000",
            "第8名: $3,000",
            "破世界纪录额外: $30,000",
        ]
    }

    /// 比赛日程
    pub fn schedule(&self) -> Vec<&'static str> {
        vec![
            "预赛: 上午进行",
            "半决赛: 当天晚上",
            "决赛: 第二天晚上",
            "公开水域: 单独日期",
            "花样游泳: 跨多日",
            "跳水: 跨多日",
        ]
    }

    /// 兴奋剂检测
    pub fn doping_control(&self) -> Vec<&'static str> {
        vec![
            "所有奖牌获得者检测",
            "随机抽检",
            "赛外检测",
            "生物护照追踪",
            "禁赛处罚: 4年起",
        ]
    }

    /// 技术官员
    pub fn technical_officials(&self) -> Vec<&'static str> {
        vec![
            "总裁判长: 1人",
            "裁判长: 4人",
            "发令员: 2人",
            "转身检查员: 每泳道2人",
            "计时员: 每泳道3人",
            "终点裁判: 4人",
        ]
    }
}

impl Default for SwimmingWorldChampionshipRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SwimmingWorldChampionshipRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("swimming_world_championship")
    }

    fn explain(&self) -> String {
        format!(
            "【世界游泳锦标赛规则】\n\n\
            比赛项目:\n{}\n\n\
            参赛资格:\n{}\n\n\
            奖金分配:\n{}\n\n\
            比赛日程:\n{}\n\n\
            兴奋剂检测:\n{}",
            self.events()
                .iter()
                .map(|e| format!("  • {}", e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.qualification()
                .iter()
                .map(|q| format!("  • {}", q))
                .collect::<Vec<_>>()
                .join("\n"),
            self.prize_money()
                .iter()
                .map(|p| format!("  • {}", p))
                .collect::<Vec<_>>()
                .join("\n"),
            self.schedule()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.doping_control()
                .iter()
                .map(|d| format!("  • {}", d))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}
