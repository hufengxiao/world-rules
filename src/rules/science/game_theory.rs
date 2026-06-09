//! 博弈论定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 博弈论定律集合
pub struct GameTheoryLaws {
    metadata: RuleMetadata,
}

impl GameTheoryLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("博弈论定律", "博弈论基本定律")
                .with_origin("数学")
                .with_tags(vec!["科学".into(), "数学".into(), "博弈".into()]),
        }
    }

    /// 基本定律
    pub fn basic_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("纳什均衡定律", "均衡状态", "无人单方面改变"),
            ("帕累托最优定律", "最优状态", "无法在不损害他人下改进"),
            ("零和博弈定律", "总和为零", "一方所得即另一方所失"),
            ("非零和博弈定律", "总和不定", "可能双赢或双输"),
            ("囚徒困境定律", "个体理性导致集体非理性", "两囚徒背叛"),
            ("理性选择定律", "理性假设", "参与者理性决策"),
            ("信息完全定律", "完全信息", "信息对称博弈"),
            ("信息不完全定律", "不完全信息", "信息不对称博弈"),
        ]
    }

    /// 策略定律
    pub fn strategy_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("最优策略定律", "策略选择", "最优响应策略"),
            ("混合策略定律", "概率组合", "以概率选择策略"),
            ("纯策略定律", "确定选择", "确定性策略"),
            ("占优策略定律", "绝对最优", "无论对手如何最优"),
            ("劣势策略定律", "劣势淘汰", "淘汰劣势策略"),
            ("威胁定律", "可信威胁", "可信威胁策略"),
            ("承诺定律", "可信承诺", "可信承诺机制"),
        ]
    }

    /// 合作博弈定律
    pub fn cooperative_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("合作博弈定律", "合作联盟", "合作形成联盟"),
            ("核心定律", "核心分配", "联盟稳定分配"),
            ("夏普利值定律", "贡献度量", "公平贡献分配"),
            ("核仁定律", "核仁解", "最小最大不满意"),
            ("联盟形成定律", "联盟博弈", "联盟结构形成"),
            ("收益分配定律", "分配机制", "联盟收益分配"),
        ]
    }

    /// 动态博弈定律
    pub fn dynamic_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("子博弈完美定律", "完美均衡", "每个子博弈纳什均衡"),
            ("逆向归纳定律", "逆向推理", "从终点逆向推理"),
            ("重复博弈定律", "重复进行", "同一博弈重复"),
            ("触发策略定律", "触发惩罚", "偏离触发惩罚"),
            ("序贯博弈定律", "顺序进行", "参与者轮流决策"),
            ("无限博弈定律", "无限重复", "无终止博弈"),
        ]
    }

    /// 博弈类型
    pub fn game_types(&self) -> Vec<&'static str> {
        vec![
            "静态博弈",
            "动态博弈",
            "完全信息博弈",
            "不完全信息博弈",
            "合作博弈",
            "非合作博弈",
            "重复博弈",
            "随机博弈",
        ]
    }

    /// 博弈应用
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "经济学",
            "政治学",
            "国际关系",
            "生物学",
            "计算机科学",
            "军事战略",
            "商业竞争",
            "体育比赛",
        ]
    }

    /// 信息博弈定律
    pub fn information_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("信号博弈定律", "信号传递", "信息优势方发送信号"),
            ("筛选博弈定律", "信息筛选", "信息劣势方设计筛选机制"),
            ("道德风险定律", "行为隐藏", "合同后隐藏行为问题"),
            ("逆向选择定律", "信息不对称", "签约前信息不对称"),
            ("拍卖博弈定律", "竞价策略", "拍卖机制与策略"),
            ("机制设计定律", "激励相容", "设计激励相容机制"),
            ("贝叶斯博弈定律", "信念更新", "不完全信息下信念更新"),
            ("共同知识定律", "公共信念", "参与者共同知识"),
        ]
    }

    /// 演化博弈定律
    pub fn evolutionary_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("演化稳定定律", "ESS策略", "演化稳定策略"),
            ("复制动态定律", "策略模仿", "成功策略被模仿扩散"),
            ("种群博弈定律", "种群互动", "大种群随机配对博弈"),
            ("利他行为定律", "利他演化", "利他行为演化解释"),
            ("鹰鸽博弈定律", "侵略与温和", "攻击与和平策略演化"),
            ("互惠定律", "直接互惠", "重复互动中互惠合作"),
            ("亲缘定律", "亲缘选择", "亲缘关系影响合作"),
            ("群体选择定律", "群体竞争", "群体层面选择压力"),
        ]
    }

    /// 社会选择定律
    pub fn social_choice_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            (
                "阿罗不可能定律",
                "投票悖论",
                "满足所有合理条件的投票规则不存在",
            ),
            ("孔多塞悖论", "循环偏好", "多数票可能产生循环"),
            ("吉巴德-萨特斯韦特定理", "策略投票", "投票策略操纵不可避免"),
            ("中位选民定律", "中间偏好", "中位选民决定结果"),
            ("布坎公共选择定律", "公共选择", "政治市场理性选择"),
            ("蒂布特模型定律", "以脚投票", "居民迁移选择社区"),
            ("奥尔森集体行动定律", "搭便车", "大集团集体行动困难"),
            ("维克里拍卖定律", "真实报价", "第二价格拍卖激励真实报价"),
        ]
    }

    /// 非合作博弈
    pub fn non_cooperative_games(&self) -> Vec<&'static str> {
        vec![
            "纳什均衡: 任何玩家单独偏离都不会改善收益",
            "囚徒困境: 个体理性导致集体非最优结果",
            "混合策略: 以概率分布随机选择纯策略",
            "子博弈完美均衡: 在每个子博弈中都构成纳什均衡",
            "贝叶斯博弈: 参与者不完全了解其他人的类型",
            "重复博弈: 同一博弈重复进行可能促进合作",
        ]
    }

    /// 机制设计
    pub fn mechanism_design(&self) -> Vec<&'static str> {
        vec![
            "显示原理: 任何机制都可以转化为直接显示机制",
            "VCG机制: 使说真话成为占优策略的拍卖机制",
            "激励相容: 个人理性行为恰好实现社会目标",
            "拍卖理论: 英式荷式密封第一价格和第二价格拍卖",
            "匹配理论: 双边市场中的稳定匹配",
            "社会选择理论: 如何从个体偏好汇总出社会偏好",
        ]
    }
}

impl Default for GameTheoryLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for GameTheoryLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("game_theory")
    }

    fn explain(&self) -> String {
        format!(
            "【博弈论定律】\n\n基本定律:\n{}\n\n策略定律:\n{}\n\n合作定律:\n{}\n\n信息博弈定律:\n{}\n\n演化博弈定律:\n{}\n\n社会选择定律:\n{}\n",
            self.basic_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.strategy_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cooperative_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.information_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.evolutionary_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.social_choice_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_theory_laws() {
        let laws = GameTheoryLaws::new();
        assert!(!laws.basic_laws().is_empty());
        assert!(!laws.strategy_laws().is_empty());
    }
}
