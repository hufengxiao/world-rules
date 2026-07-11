//! 植树节礼仪 - 中国植树节礼仪规范
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! { struct: ArborDayRules, name: "植树节礼仪", desc: "中国植树节礼仪规范", origin: "中国", tags: ["社交", "节日", "环保"] }

impl ArborDayRules {
    /// 节日意义
    pub fn significance(&self) -> Vec<&'static str> {
        vec![
            "绿化环境 - 增加森林覆盖率，改善生态环境",
            "保护生态 - 保护生物多样性，维护生态平衡",
            "防风固沙 - 防止土地荒漠化，治理沙尘暴",
            "净化空气 - 吸收二氧化碳，释放氧气",
            "纪念孙中山 - 纪念孙中山先生倡导植树造林",
        ]
    }

    /// 植树礼仪
    pub fn planting_etiquette(&self) -> Vec<&'static str> {
        vec![
            "科学选址 - 选择适宜树木生长的地点",
            "合理选种 - 选择适合当地气候的树种",
            "规范种植 - 按照正确方法种植树苗",
            "精心护理 - 种植后做好浇水施肥工作",
            "保护树木 - 保护树木不受损害",
            "参与集体植树 - 积极参加单位组织的植树活动",
        ]
    }

    /// 环保行动
    pub fn environmental_actions(&self) -> Vec<&'static str> {
        vec![
            "节约用纸 - 减少纸张浪费，保护森林",
            "减少一次性用品 - 减少使用一次性筷子等",
            "回收利用 - 回收纸张和木质制品",
            "绿色出行 - 选择环保的出行方式",
            "爱护花草 - 爱护公共场所的花草树木",
            "参与环保活动 - 积极参与各类环保活动",
        ]
    }

    /// 社会参与
    pub fn social_participation(&self) -> Vec<&'static str> {
        vec![
            "政府组织 - 政府组织全民义务植树活动",
            "企业参与 - 企业组织员工参与植树",
            "学校活动 - 学校组织学生参加植树",
            "社区行动 - 社区组织居民植树绿化",
            "志愿活动 - 志愿者参与植树造林",
            "认养树木 - 认养并照顾公共树木",
        ]
    }

    /// 日常环保礼仪
    pub fn daily_etiquette(&self) -> Vec<&'static str> {
        vec![
            "不践踏草坪 - 不踩踏公共绿地",
            "不攀折花木 - 不攀爬或折断树枝",
            "不乱刻乱画 - 不在树上刻字或画图",
            "节约用水 - 节约水资源保护环境",
            "垃圾分类 - 正确分类投放垃圾",
            "宣传环保 - 向他人宣传环保知识",
        ]
    }

    /// 祝福用语
    pub fn greetings(&self) -> Vec<&'static str> {
        vec![
            "植树节快乐 - 节日祝福",
            "绿水青山就是金山银山 - 强调环保重要性",
            "让地球充满绿色 - 祝愿环境美好",
            "播种绿色收获希望 - 表达植树意义",
            "十年树木百年树人 - 引申植树的教育意义",
            "保护环境人人有责 - 强调环保责任",
        ]
    }
}

impl Rule for ArborDayRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("arbor_day")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "植树节礼仪",
            &[
                ("节日意义", &self.significance()),
                ("植树礼仪", &self.planting_etiquette()),
                ("环保行动", &self.environmental_actions()),
                ("社会参与", &self.social_participation()),
                ("日常礼仪", &self.daily_etiquette()),
                ("祝福用语", &self.greetings()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arbor_day_rules() {
        let rules = ArborDayRules::new();
        assert_eq!(rules.metadata().name, "植树节礼仪");
        assert!(!rules.explain().is_empty());
        assert!(rules.significance().len() >= 5);
        assert!(rules.planting_etiquette().len() >= 5);
        assert!(rules.environmental_actions().len() >= 5);
        assert!(rules.social_participation().len() >= 5);
    }
}
