//! 青少年游泳规则
//!
//! 针对不同年龄段青少年的游泳运动规则，包括项目调整、安全要求等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 青少年游泳年龄组别
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YouthSwimAgeGroup {
    /// 少儿组（6-8岁）
    Minnows,
    /// 儿童组（9-10岁）
    Children,
    /// 少年组（11-12岁）
    Youth,
    /// 青少年组（13-14岁）
    Junior,
    /// 青年组（15-17岁）
    YoungAdult,
}

impl YouthSwimAgeGroup {
    /// 获取年龄组名称
    pub fn name(&self) -> &'static str {
        match self {
            YouthSwimAgeGroup::Minnows => "少儿组",
            YouthSwimAgeGroup::Children => "儿童组",
            YouthSwimAgeGroup::Youth => "少年组",
            YouthSwimAgeGroup::Junior => "青少年组",
            YouthSwimAgeGroup::YoungAdult => "青年组",
        }
    }

    /// 获取年龄范围
    pub fn age_range(&self) -> &'static str {
        match self {
            YouthSwimAgeGroup::Minnows => "6-8岁",
            YouthSwimAgeGroup::Children => "9-10岁",
            YouthSwimAgeGroup::Youth => "11-12岁",
            YouthSwimAgeGroup::Junior => "13-14岁",
            YouthSwimAgeGroup::YoungAdult => "15-17岁",
        }
    }
}

/// 青少年游泳规则
pub struct YouthSwimmingRules {
    metadata: RuleMetadata,
}

impl YouthSwimmingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("青少年游泳规则", "青少年游泳运动规则和年龄分组")
                .with_origin("FINA/中国泳协")
                .with_tags(vec![
                    "体育".into(),
                    "游泳".into(),
                    "青少年".into(),
                    "校园".into(),
                ]),
        }
    }

    /// 年龄分组标准
    pub fn age_classifications(&self) -> Vec<&'static str> {
        vec![
            "少儿组（Minnows）: 6-8岁",
            "儿童组（Children）: 9-10岁",
            "少年组（Youth）: 11-12岁",
            "青少年组（Junior）: 13-14岁",
            "青年组（Young Adult）: 15-17岁",
            "年龄验证: 需提供出生证明",
        ]
    }

    /// 少儿组（6-8岁）项目设置
    pub fn minnows_events(&self) -> Vec<&'static str> {
        vec![
            "25米自由泳",
            "25米蛙泳",
            "25米仰泳",
            "4×25米自由泳接力",
            "不设蝶泳项目",
            "不设混合泳项目",
            "可使用辅助浮板",
        ]
    }

    /// 儿童组（9-10岁）项目设置
    pub fn children_events(&self) -> Vec<&'static str> {
        vec![
            "50米自由泳、蛙泳、仰泳",
            "100米自由泳、蛙泳",
            "4×50米自由泳接力",
            "可尝试蝶泳（25米）",
            "不设长距离项目",
        ]
    }

    /// 少年组（11-12岁）项目设置
    pub fn youth_events(&self) -> Vec<&'static str> {
        vec![
            "50米、100米各泳姿",
            "200米自由泳、混合泳",
            "蝶泳正式设项",
            "4×50米、4×100米接力",
            "可参加长距离训练",
        ]
    }

    /// 青少年组（13-14岁）项目设置
    pub fn junior_events(&self) -> Vec<&'static str> {
        vec![
            "接近成人项目设置",
            "50米、100米、200米各泳姿",
            "400米、800米自由泳",
            "200米、400米混合泳",
            "各项接力比赛",
        ]
    }

    /// 青年组（15-17岁）项目设置
    pub fn young_adult_events(&self) -> Vec<&'static str> {
        vec![
            "成人标准项目",
            "1500米自由泳可选",
            "男子1500米、女子800米长距离",
            "接力项目完整设置",
            "可参加成人组比赛",
        ]
    }

    /// 泳池规格要求
    pub fn pool_requirements(&self) -> Vec<&'static str> {
        vec![
            "标准池: 50米长池",
            "短池: 25米",
            "少儿组可使用25米池",
            "水深: 至少1.35米",
            "水温: 25-28°C",
            "必须配备救生员",
            "每泳道配备观察员",
        ]
    }

    /// 出发规则（按年龄组）
    pub fn starting_rules(&self, age_group: YouthSwimAgeGroup) -> Vec<&'static str> {
        match age_group {
            YouthSwimAgeGroup::Minnows => {
                vec![
                    "水中出发",
                    "教练可协助准备",
                    "不设出发跳台",
                    "允许扶池边",
                ]
            }
            YouthSwimAgeGroup::Children => {
                vec![
                    "自由泳、蛙泳: 可跳台出发",
                    "仰泳: 水中出发",
                    "教练可辅助指导",
                    "允许使用出发辅助器",
                ]
            }
            YouthSwimAgeGroup::Youth => {
                vec![
                    "学习跳台出发技术",
                    "仰泳: 水中出发",
                    "可申请水中出发",
                    "抢跳警告制度",
                ]
            }
            YouthSwimAgeGroup::Junior | YouthSwimAgeGroup::YoungAdult => {
                vec![
                    "标准跳台出发",
                    "仰泳: 水中背向出发",
                    "抢跳: 取消资格",
                    "符合成人标准",
                ]
            }
        }
    }

    /// 转身规则（按年龄组）
    pub fn turn_rules(&self, age_group: YouthSwimAgeGroup) -> Vec<&'static str> {
        match age_group {
            YouthSwimAgeGroup::Minnows | YouthSwimAgeGroup::Children => {
                vec![
                    "双手触壁转身",
                    "允许转身辅助",
                    "不要求滚翻转身",
                    "简化转身技术",
                ]
            }
            YouthSwimAgeGroup::Youth => {
                vec![
                    "学习滚翻转身",
                    "蛙泳、蝶泳: 双手触壁",
                    "自由泳: 可尝试滚翻",
                    "教练可现场指导",
                ]
            }
            YouthSwimAgeGroup::Junior | YouthSwimAgeGroup::YoungAdult => {
                vec![
                    "自由泳、仰泳: 滚翻转身",
                    "蛙泳、蝶泳: 双手触壁",
                    "混合泳: 按泳姿规则",
                    "符合成人标准",
                ]
            }
        }
    }

    /// 安全保护措施
    pub fn safety_measures(&self) -> Vec<&'static str> {
        vec![
            "赛前体检: 必须提供健康证明",
            "救生员配置: 每泳道至少1名",
            "医疗站: 必须配备医护人员",
            "急救设备: AED、担架、氧气",
            "人数限制: 每泳道不超过8人",
            "水温监测: 过冷/过热暂停",
            "深水区保护: 不熟水性者穿戴救生衣",
            "禁止: 无救生员时下水",
        ]
    }

    /// 参赛资格要求
    pub fn eligibility_requirements(&self) -> Vec<&'static str> {
        vec![
            "年龄证明: 出生证明或身份证",
            "游泳能力测试: 通过基本技能测试",
            "健康证明: 近期体检报告",
            "家长同意书: 未满18岁必须",
            "保险: 意外伤害保险",
            "教练推荐: 如需要",
        ]
    }

    /// 训练建议
    pub fn training_guidelines(&self) -> Vec<&'static str> {
        vec![
            "训练时长: 每周不超过8小时",
            "多样化训练: 避免过早专项化",
            "技术优先: 注重动作规范",
            "强度控制: 避免过度训练",
            "休息恢复: 保证充足睡眠",
            "营养指导: 科学饮食搭配",
            "心理健康: 关注心理状态",
            "趣味性: 保持训练兴趣",
        ]
    }

    /// 比赛服装要求
    pub fn uniform_requirements(&self) -> Vec<&'static str> {
        vec![
            "标准泳衣/泳裤",
            "泳帽: 必须佩戴",
            "泳镜: 建议佩戴",
            "禁止: 潜水服、脚蹼",
            "禁止: 手蹼、划水板（比赛时）",
            "年龄组规定: 泳衣不能覆盖颈部",
        ]
    }

    /// 犯规判定
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "抢跳: 两次抢跳取消资格",
            "泳姿违规: 不符合技术规则",
            "触壁违规: 单手触壁（蛙/蝶）",
            "转身违规: 未触壁转身",
            "串道: 干扰其他泳道",
            "行走: 蹬池底行走",
            "接力违规: 提前入水",
            "不当行为: 不尊重裁判",
        ]
    }

    /// 成绩记录规则
    pub fn record_rules(&self) -> Vec<&'static str> {
        vec![
            "电子计时: 精确到0.01秒",
            "手计时: 精确到0.1秒",
            "分段成绩: 接力需记录",
            "成绩公示: 赛后及时公布",
            "录像回放: 争议判罚依据",
            "申诉流程: 赛后30分钟内",
        ]
    }

    /// 特殊规则调整
    pub fn special_adjustments(&self) -> Vec<&'static str> {
        vec![
            "少儿组: 可使用浮板（不计成绩）",
            "儿童组: 允许起跳辅助",
            "允许教练在池边指导（不干扰比赛）",
            "增加休息时间（项目间至少30分钟）",
            "心理辅导: 关注紧张情绪",
            "热身时间延长",
        ]
    }
}

impl Default for YouthSwimmingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for YouthSwimmingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("youth_swimming")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        format!(
            "【青少年游泳规则】\n\n\
            年龄分组标准:\n{}\n\n\
            少儿组（6-8岁）项目:\n{}\n\n\
            儿童组（9-10岁）项目:\n{}\n\n\
            少年组（11-12岁）项目:\n{}\n\n\
            青少年组（13-14岁）项目:\n{}\n\n\
            安全保护措施:\n{}",
            self.age_classifications()
                .iter()
                .map(|c| format!("  • {}", c))
                .collect::<Vec<_>>()
                .join("\n"),
            self.minnows_events()
                .iter()
                .map(|c| format!("  • {}", c))
                .collect::<Vec<_>>()
                .join("\n"),
            self.children_events()
                .iter()
                .map(|c| format!("  • {}", c))
                .collect::<Vec<_>>()
                .join("\n"),
            self.youth_events()
                .iter()
                .map(|c| format!("  • {}", c))
                .collect::<Vec<_>>()
                .join("\n"),
            self.junior_events()
                .iter()
                .map(|c| format!("  • {}", c))
                .collect::<Vec<_>>()
                .join("\n"),
            self.safety_measures()
                .iter()
                .map(|c| format!("  • {}", c))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_youth_swimming_basic() {
        let rules = YouthSwimmingRules::new();
        assert_eq!(rules.metadata().name, "青少年游泳规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_age_groups() {
        let rules = YouthSwimmingRules::new();
        let classifications = rules.age_classifications();
        assert!(classifications.len() >= 5);
        assert!(classifications.iter().any(|c| c.contains("少儿组")));
        assert!(classifications.iter().any(|c| c.contains("儿童组")));
        assert!(classifications.iter().any(|c| c.contains("少年组")));
    }

    #[test]
    fn test_minnows_events() {
        let rules = YouthSwimmingRules::new();
        let events = rules.minnows_events();
        assert!(events.iter().any(|e| e.contains("25米")));
        assert!(events.iter().any(|e| e.contains("不设蝶泳")));
    }

    #[test]
    fn test_children_events() {
        let rules = YouthSwimmingRules::new();
        let events = rules.children_events();
        assert!(events.iter().any(|e| e.contains("50米")));
        assert!(events.iter().any(|e| e.contains("蛙泳")));
    }

    #[test]
    fn test_youth_events() {
        let rules = YouthSwimmingRules::new();
        let events = rules.youth_events();
        assert!(events.iter().any(|e| e.contains("蝶泳")));
        assert!(events.iter().any(|e| e.contains("混合泳")));
    }

    #[test]
    fn test_starting_rules() {
        let rules = YouthSwimmingRules::new();
        
        // 少儿组水中出发
        let minnows_start = rules.starting_rules(YouthSwimAgeGroup::Minnows);
        assert!(minnows_start.iter().any(|s| s.contains("水中出发")));
        
        // 青少年组跳台出发
        let junior_start = rules.starting_rules(YouthSwimAgeGroup::Junior);
        assert!(junior_start.iter().any(|s| s.contains("跳台")));
    }

    #[test]
    fn test_safety_measures() {
        let rules = YouthSwimmingRules::new();
        let safety = rules.safety_measures();
        assert!(safety.iter().any(|s| s.contains("救生员")));
        assert!(safety.iter().any(|s| s.contains("医疗")));
        assert!(safety.len() >= 6);
    }

    #[test]
    fn test_category() {
        let rules = YouthSwimmingRules::new();
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }

    #[test]
    fn test_age_group_enum() {
        assert_eq!(YouthSwimAgeGroup::Minnows.name(), "少儿组");
        assert_eq!(YouthSwimAgeGroup::Children.age_range(), "9-10岁");
        assert_eq!(YouthSwimAgeGroup::Junior.name(), "青少年组");
    }

    #[test]
    fn test_pool_requirements() {
        let rules = YouthSwimmingRules::new();
        let pool = rules.pool_requirements();
        assert!(pool.iter().any(|p| p.contains("标准池")));
        assert!(pool.iter().any(|p| p.contains("水深")));
    }

    #[test]
    fn test_fouls() {
        let rules = YouthSwimmingRules::new();
        let fouls = rules.fouls();
        assert!(fouls.iter().any(|f| f.contains("抢跳")));
        assert!(fouls.iter().any(|f| f.contains("泳姿")));
    }
}