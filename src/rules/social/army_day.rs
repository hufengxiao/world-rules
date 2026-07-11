//! 建军节礼仪 - 中国人民解放军建军节礼仪规范
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! { struct: ArmyDayRules, name: "建军节礼仪", desc: "中国人民解放军建军节礼仪规范", origin: "中国", tags: ["社交", "节日", "军事"] }

impl ArmyDayRules {
    /// 节日意义
    pub fn significance(&self) -> Vec<&'static str> {
        vec![
            "纪念建军 - 纪念中国人民解放军建军",
            "弘扬军魂 - 弘扬人民军队的光荣传统",
            "拥军优属 - 关心爱护军人和军属",
            "国防教育 - 开展全民国防教育",
            "致敬军人 - 向人民子弟兵致敬",
        ]
    }

    /// 军队礼仪
    pub fn military_etiquette(&self) -> Vec<&'static str> {
        vec![
            "升旗仪式 - 举行升国旗仪式",
            "阅兵式 - 组织阅兵活动",
            "表彰大会 - 表彰优秀军人",
            "慰问官兵 - 慰问驻地官兵",
            "军营开放日 - 开放军营供公众参观",
            "军史展览 - 举办军队历史展览",
        ]
    }

    /// 民间礼仪
    pub fn civilian_etiquette(&self) -> Vec<&'static str> {
        vec![
            "拥军慰问 - 慰问军人和军属",
            "参观展览 - 参观军事博物馆",
            "观看演出 - 观看建军节文艺演出",
            "学习国防知识 - 学习国防和军事知识",
            "致敬军人 - 向军人表达敬意",
            "关心退伍军人 - 关心和帮助退伍军人",
        ]
    }

    /// 军人礼仪
    pub fn soldier_etiquette(&self) -> Vec<&'static str> {
        vec![
            "忠诚于党 - 坚持党对军队的绝对领导",
            "服务人民 - 全心全意为人民服务",
            "英勇善战 - 敢于战斗、善于战斗",
            "严守纪律 - 严格遵守军队纪律",
            "刻苦训练 - 刻苦训练提高本领",
            "维护荣誉 - 维护军队和军人荣誉",
        ]
    }

    /// 军属礼仪
    pub fn military_family_etiquette(&self) -> Vec<&'static str> {
        vec![
            "支持服役 - 支持家人安心服役",
            "保持荣誉 - 维护军人家庭的荣誉",
            "联系沟通 - 保持与服役军人的联系",
            "关心照顾 - 关心照顾军人家属",
            "解决困难 - 帮助解决军属实际困难",
            "社区关爱 - 社区关心关爱军属",
        ]
    }

    /// 祝福用语
    pub fn greetings(&self) -> Vec<&'static str> {
        vec![
            "建军节快乐 - 最常用的节日祝福",
            "向人民子弟兵致敬 - 表达对军人的敬意",
            "军旗飘扬 - 赞美军队威武",
            "钢铁长城 - 赞美军队保卫国家",
            "保家卫国 - 肯定军人的贡献",
            "军魂永驻 - 祝愿军队永远强大",
        ]
    }
}

impl Rule for ArmyDayRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("army_day")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "建军节礼仪",
            &[
                ("节日意义", &self.significance()),
                ("军队礼仪", &self.military_etiquette()),
                ("民间礼仪", &self.civilian_etiquette()),
                ("军人礼仪", &self.soldier_etiquette()),
                ("军属礼仪", &self.military_family_etiquette()),
                ("祝福用语", &self.greetings()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_army_day_rules() {
        let rules = ArmyDayRules::new();
        assert_eq!(rules.metadata().name, "建军节礼仪");
        assert!(!rules.explain().is_empty());
        assert!(rules.significance().len() >= 5);
        assert!(rules.military_etiquette().len() >= 5);
        assert!(rules.civilian_etiquette().len() >= 5);
        assert!(rules.soldier_etiquette().len() >= 5);
    }
}
