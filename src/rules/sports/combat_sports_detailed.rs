//! 格斗项目详细规则
//!
//! 本模块提供格斗运动的详细规则实现，涵盖拳击、摔跤、柔道、跆拳道、击剑等。
//! 符合各国际单项体育联合会标准。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 格斗运动类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatSport {
    /// 拳击
    Boxing,
    /// 摔跤
    Wrestling,
    /// 柔道
    Judo,
    /// 跆拳道
    Taekwondo,
    /// 击剑
    Fencing,
    /// 空手道
    Karate,
    /// 泰拳
    MuayThai,
    /// 柔术
    BrazilianJiuJitsu,
}

impl CombatSport {
    /// 获取运动名称
    pub fn name(&self) -> &'static str {
        match self {
            CombatSport::Boxing => "拳击",
            CombatSport::Wrestling => "摔跤",
            CombatSport::Judo => "柔道",
            CombatSport::Taekwondo => "跆拳道",
            CombatSport::Fencing => "击剑",
            CombatSport::Karate => "空手道",
            CombatSport::MuayThai => "泰拳",
            CombatSport::BrazilianJiuJitsu => "巴西柔术",
        }
    }

    /// 是否为奥运会项目
    pub fn is_olympic(&self) -> bool {
        matches!(
            self,
            CombatSport::Boxing
                | CombatSport::Wrestling
                | CombatSport::Judo
                | CombatSport::Taekwondo
                | CombatSport::Fencing
                | CombatSport::Karate
        )
    }

    /// 是否允许打击
    pub fn allows_striking(&self) -> bool {
        matches!(
            self,
            CombatSport::Boxing
                | CombatSport::Taekwondo
                | CombatSport::Karate
                | CombatSport::MuayThai
        )
    }

    /// 是否允许摔投
    pub fn allows_grappling(&self) -> bool {
        matches!(
            self,
            CombatSport::Wrestling | CombatSport::Judo | CombatSport::BrazilianJiuJitsu
        )
    }
}

/// 格斗项目详细规则
pub struct CombatSportsDetailedRules {
    metadata: RuleMetadata,
}

impl CombatSportsDetailedRules {
    /// 创建新的格斗项目详细规则实例
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("格斗项目详细规则", "格斗运动详细比赛规则")
                .with_origin("各国际单项体育联合会")
                .with_tags(vec!["体育".into(), "格斗".into()]),
        }
    }

    // ==================== 拳击规则 ====================

    /// 拳击比赛规则
    pub fn boxing_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛回合: 男子3分钟×12回合，女子2分钟×10回合",
            "体重级别: 男子8个级别（职业）",
            "拳套重量: 8盎司（职业）、10盎司（业余）",
            "场地: 4.9-6.1米正方形拳台",
            "得分规则: 10分制评分系统",
            "击打部位: 头部和躯干（正面和侧面）",
            "禁止击打: 后脑、下体、背部",
            "裁判: 3名裁判评分",
        ]
    }

    /// 拳击体重级别
    pub fn boxing_weight_classes(&self) -> Vec<&'static str> {
        vec![
            "蝇量级: 50.8公斤以下",
            "最轻量级: 53.5公斤",
            "羽量级: 57.2公斤",
            "轻量级: 61.2公斤",
            "次中量级: 66.7公斤",
            "中量级: 72.6公斤",
            "超中量级: 76.2公斤",
            "重量级: 90.7公斤以上",
        ]
    }

    /// 拳击犯规行为
    pub fn boxing_fouls(&self) -> Vec<&'static str> {
        vec![
            "击打下体",
            "击打后脑",
            "头部冲撞",
            "肘击",
            "开掌击打",
            "搂抱过度",
            "击打倒地对手",
            "故意吐出护齿",
        ]
    }

    /// 拳击获胜方式
    pub fn boxing_victory_methods(&self) -> Vec<&'static str> {
        vec![
            "KO（击倒）: 对手10秒内无法起身",
            "TKO（技术击倒）: 裁判终止比赛",
            "判定获胜: 回合结束后得分高者",
            "弃权: 对手主动弃权",
            "取消资格: 对手严重犯规",
            "多数判定: 2名裁判判胜，1名判平",
            "一致判定: 3名裁判一致判胜",
            "平局: 双方得分相同",
        ]
    }

    /// 业余拳击规则
    pub fn amateur_boxing_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛回合: 男子3回合×3分钟，女子4回合×2分钟",
            "拳套重量: 10盎司（统一）",
            "头盔: 必须佩戴（男子）",
            "评分系统: 电子计分系统",
            "得分点: 有效击打头部或躯干",
            "体重级别: 13个级别（奥运会）",
            "裁判: 5名裁判（3名评分）",
            "警告: 3次警告取消资格",
        ]
    }

    // ==================== 摔跤规则 ====================

    /// 摔跤比赛规则
    pub fn wrestling_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛时间: 2个3分钟回合",
            "比赛类型: 自由式、古典式",
            "体重级别: 男子10个级别（奥运会）",
            "场地: 直径9米的圆形垫子",
            "得分方式: 摔倒、翻滚、控制",
            "胜利条件: 技术摔倒（10分差距）",
            "加时赛: 平局时进行加时赛",
            "裁判: 1名主裁判、2名副裁判",
        ]
    }

    /// 古典式摔跤规则
    pub fn greco_roman_rules(&self) -> Vec<&'static str> {
        vec![
            "特点: 只允许攻击上半身",
            "禁止动作: 攻击对手腿部",
            "技术特点: 摔投、抱摔",
            "站立摔: 从站立姿势开始",
            "跪撑摔: 对手跪地时攻击",
            "得分动作: 大动作（5分）、小动作（1-3分）",
            "被动处罚: 被动方需进攻",
            "服装: 紧身连体摔跤服",
        ]
    }

    /// 自由式摔跤规则
    pub fn freestyle_wrestling_rules(&self) -> Vec<&'static str> {
        vec![
            "特点: 允许攻击全身",
            "技术特点: 腿部攻击、摔投",
            "得分动作: 技术摔倒、翻滚、控制",
            "大动作: 5分（直接将对手摔成危险姿态）",
            "小动作: 1-3分（根据技术质量）",
            "被动处罚: 被动方需进攻",
            "警告: 犯规动作、消极比赛",
            "加时赛: 平局时优先判定技术分",
        ]
    }

    /// 摔跤得分规则
    pub fn wrestling_scoring(&self) -> Vec<&'static str> {
        vec![
            "5分: 大动作（直接摔成危险姿态）",
            "3分: 将对手摔成危险姿态",
            "2分: 翻滚对手、控制对手",
            "1分: 小动作、推出场外",
            "技术摔倒: 领先10分直接获胜",
            "警告: 犯规扣1分",
            "被动警告: 第1次口头警告",
            "取消资格: 3次警告",
        ]
    }

    // ==================== 柔道规则 ====================

    /// 柔道比赛规则
    pub fn judo_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛时间: 4分钟（男）、4分钟（女）",
            "体重级别: 男子7个级别、女子7个级别",
            "场地: 14-16米×14-16米正方形垫子",
            "比赛区域: 8-10米×8-10米内场",
            "服装: 柔道服（上衣、裤子、腰带）",
            "腰带颜色: 白色（一方）、蓝色或黑色（另一方）",
            "裁判: 1名主裁判、2名副裁判",
            "视频回放: 允许使用VAR系统",
        ]
    }

    /// 柔道得分规则
    pub fn judo_scoring(&self) -> Vec<&'static str> {
        vec![
            "一本: 直接获胜（完美摔投）",
            "一本条件: 对手背部着地、力量、速度、控制",
            "技有: 部分成功的摔投（2个技有 = 1个一本）",
            "有效: 更小程度的摔投（1个技有 = 3个有效）",
            "压制: 20秒 = 一本，15-19秒 = 技有",
            "犯规: 指导、警告、取消资格",
            "金分加时: 平局时进行无限时加时",
            "综合胜利: 分数累加获胜",
        ]
    }

    /// 柔道犯规行为
    pub fn judo_penalties(&self) -> Vec<&'static str> {
        vec![
            "指导（轻微犯规）: 累计3次 = 犯规",
            "消极比赛: 不主动进攻",
            "禁止动作: 攻击关节、颈部",
            "危险动作: 可能伤害对手",
            "非武士道行为: 不尊重对手",
            "服装违规: 柔道服不合格",
            "场外比赛: 故意出界",
            "取消资格: 严重犯规",
        ]
    }

    /// 柔道技术
    pub fn judo_techniques(&self) -> Vec<&'static str> {
        vec![
            "投技: 站立摔投技术",
            "手技: 用手臂投摔",
            "腰技: 用腰部投摔",
            "足技: 用腿部投摔",
            "真舍身技: 主动倒地投摔",
            "横舍身技: 横向投摔",
            "固技: 地面控制技术",
            "关节技: 关节控制技术",
        ]
    }

    // ==================== 跆拳道规则 ====================

    /// 跆拳道比赛规则
    pub fn taekwondo_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛回合: 3回合×2分钟",
            "体重级别: 男子4个级别、女子4个级别（奥运会）",
            "场地: 8米×8米正方形垫子",
            "比赛形式: 对打（竞技）、品势（表演）",
            "服装: 跆拳道服、头盔、护具",
            "电子护具: 踢击自动计分",
            "裁判: 1名主裁判、4名副裁判",
            "视频挑战: 每场比赛1次挑战机会",
        ]
    }

    /// 跆拳道得分规则
    pub fn taekwondo_scoring(&self) -> Vec<&'static str> {
        vec![
            "躯干踢击: 2分（护具区域）",
            "头部踢击: 3分",
            "旋转踢击: 额外加1分",
            "击倒: 额外加1分",
            "犯规: 警告1次扣1分",
            "获胜条件: 3回合后得分高者",
            "比分差距: 20分差距自动获胜",
            "金分加时: 平局时进行加时赛",
        ]
    }

    /// 跆拳道犯规行为
    pub fn taekwondo_fouls(&self) -> Vec<&'static str> {
        vec![
            "出界: 双脚出界",
            "倒地: 非踢击倒地",
            "消极比赛: 不主动进攻",
            "推搡: 用手推搡对手",
            "抓抱: 抓住对手",
            "低踢: 踢击腰部以下",
            "攻击后背: 攻击对手背部",
            "头部攻击: 用拳攻击头部",
        ]
    }

    /// 跆拳道体重级别
    pub fn taekwondo_weight_classes(&self) -> Vec<&'static str> {
        vec![
            "男子58公斤级",
            "男子68公斤级",
            "男子80公斤级",
            "男子80公斤以上级",
            "女子49公斤级",
            "女子57公斤级",
            "女子67公斤级",
            "女子67公斤以上级",
        ]
    }

    // ==================== 击剑规则 ====================

    /// 击剑比赛规则
    pub fn fencing_rules(&self) -> Vec<&'static str> {
        vec![
            "剑种: 花剑、重剑、佩剑",
            "比赛形式: 个人赛、团体赛",
            "比赛时间: 3分钟×3回合（个人）",
            "场地: 14米长、1.5-2米宽剑道",
            "得分: 有效击中得分",
            "得分显示: 电子计分系统",
            "裁判: 1名主裁判",
            "视频回放: 允许使用VAR系统",
        ]
    }

    /// 花剑规则
    pub fn foil_rules(&self) -> Vec<&'static str> {
        vec![
            "特点: 刺击武器，轻剑",
            "有效部位: 躯干（前胸、后背）",
            "得分: 刺中有效部位得分",
            "优先权: 攻击方优先得分",
            "反击: 需先防守再反击",
            "剑重: 不超过500克",
            "剑长: 不超过110厘米",
            "护手盘: 圆形（直径9.5-12厘米）",
        ]
    }

    /// 重剑规则
    pub fn epee_rules(&self) -> Vec<&'static str> {
        vec![
            "特点: 刺击武器，重剑",
            "有效部位: 全身（包括头部、四肢）",
            "得分: 刺中任何部位得分",
            "优先权: 无优先权，先击中者得分",
            "双击: 双方同时击中均得分",
            "剑重: 不超过770克",
            "剑长: 不超过110厘米",
            "护手盘: 三角形",
        ]
    }

    /// 佩剑规则
    pub fn sabre_rules(&self) -> Vec<&'static str> {
        vec![
            "特点: 砍击和刺击武器",
            "有效部位: 腰部以上（躯干、手臂、头部）",
            "得分: 砍中或刺中有效部位得分",
            "优先权: 攻击方优先得分",
            "反击: 需先防守再反击",
            "剑重: 不超过500克",
            "剑长: 不超过105厘米",
            "护手盘: 三角形（连接剑柄）",
        ]
    }

    /// 击剑比赛形式
    pub fn fencing_competition_formats(&self) -> Vec<&'static str> {
        vec![
            "小组赛: 每人5分×3分钟",
            "直接淘汰赛: 15分×3回合",
            "团体赛: 3人轮换，45分×9回合",
            "种子排名: 根据世界排名",
            "奥运资格: 世界排名+洲际资格赛",
            "世界排名: 积分系统",
            "积分赛事: 世界杯、世锦赛、大奖赛",
            "奥运形式: 个人+团体",
        ]
    }

    // ==================== 空手道规则 ====================

    /// 空手道比赛规则
    pub fn karate_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛形式: 组手（对打）、型（表演）",
            "组手回合: 3分钟",
            "组手场地: 8米×8米正方形",
            "型场地: 12米×12米正方形",
            "服装: 空手道服",
            "护具: 手套、护齿、护脚",
            "裁判: 1名主裁判、4名副裁判",
            "得分显示: 电子计分系统",
        ]
    }

    /// 空手道组手得分规则
    pub fn karate_scoring(&self) -> Vec<&'static str> {
        vec![
            "三分（萨邦）: 有效踢击头部",
            "二分（尼蓬）: 有效踢击躯干",
            "一分（伊蓬）: 有效拳击",
            "有效定义: 技术正确、力量控制",
            "犯规: 过度力量、危险动作",
            "获胜条件: 8分领先或时间到",
            "平局: 裁判判定技术优势",
            "取消资格: 严重犯规",
        ]
    }

    /// 空手道型规则
    pub fn karate_kata_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛形式: 表演规定或自选型",
            "评分: 技术分 + 运动分",
            "技术分: 动作正确性、力量、速度",
            "运动分: 节奏、呼吸、专注",
            "分数范围: 5.0-10.0分",
            "裁判: 7名裁判",
            "计分: 去掉最高分和最低分",
            "时间限制: 无具体时间限制",
        ]
    }

    // ==================== 通用规则 ====================

    /// 比赛装备要求
    pub fn equipment_requirements(&self) -> Vec<&'static str> {
        vec![
            "拳击: 拳套、护齿、头盔（业余）",
            "摔跤: 摔跤服、摔跤鞋",
            "柔道: 柔道服、腰带",
            "跆拳道: 跆拳道服、头盔、护具",
            "击剑: 击剑服、面罩、手套",
            "空手道: 空手道服、手套、护具",
            "护具标准: 符合国际认证",
            "服装颜色: 根据比赛规定",
        ]
    }

    /// 比赛流程
    pub fn competition_procedure(&self) -> Vec<&'static str> {
        vec![
            "检录: 赛前30分钟检录",
            "称重: 赛前2小时称重",
            "热身: 规定时间热身",
            "入场: 听到信号后入场",
            "敬礼: 向对手和裁判敬礼",
            "比赛: 根据规则进行比赛",
            "结束: 裁判判定胜负",
            "退场: 向对手和裁判致谢",
        ]
    }

    /// 反兴奋剂规定
    pub fn anti_doping_rules(&self) -> Vec<&'static str> {
        vec![
            "遵守WADA反兴奋剂条例",
            "禁药清单: WADA年度发布",
            "检测: 赛内和赛外均可检测",
            "治疗用药豁免: 需提前申请TUE",
            "行踪申报: 精英运动员需申报",
            "违规处罚: 禁赛2-4年",
            "申诉: CAS仲裁",
            "减重禁药: 利尿剂禁用",
        ]
    }

    /// 年龄组别
    pub fn age_categories(&self) -> Vec<&'static str> {
        vec![
            "青年组: 16-18岁",
            "成年组: 18岁以上",
            "年龄限制: 根据比赛级别确定",
            "青年比赛: 世界青年锦标赛",
            "成年比赛: 世界锦标赛、奥运会",
            "年龄证明: 需提供身份证明",
            "体重限制: 各年龄组体重级别不同",
            "青年规则: 部分规则有调整",
        ]
    }
}

impl Default for CombatSportsDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CombatSportsDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("combat_sports_detailed")
    }

    fn explain(&self) -> String {
        format!(
            "【格斗项目详细规则】\n\
            涵盖拳击、摔跤、柔道、跆拳道、击剑、空手道等格斗运动\n\n\
            拳击规则:\n{}\n\n\
            摔跤规则:\n{}\n\n\
            柔道规则:\n{}\n\n\
            跆拳道规则:\n{}\n\n\
            击剑规则:\n{}\n",
            self.boxing_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.wrestling_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.judo_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.taekwondo_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.fencing_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combat_sports_detailed_rules_creation() {
        let rules = CombatSportsDetailedRules::new();
        assert_eq!(rules.metadata().name, "格斗项目详细规则");
    }

    #[test]
    fn test_combat_sport_properties() {
        assert!(CombatSport::Boxing.is_olympic());
        assert!(CombatSport::Boxing.allows_striking());
        assert!(!CombatSport::Boxing.allows_grappling());
        assert!(CombatSport::Judo.allows_grappling());
        assert!(!CombatSport::Judo.allows_striking());
    }

    #[test]
    fn test_boxing_rules() {
        let rules = CombatSportsDetailedRules::new();
        let rules_list = rules.boxing_rules();
        assert!(!rules_list.is_empty());
        assert!(rules_list.iter().any(|r| r.contains("回合")));
    }

    #[test]
    fn test_judo_scoring() {
        let rules = CombatSportsDetailedRules::new();
        let scoring = rules.judo_scoring();
        assert!(!scoring.is_empty());
        assert!(scoring.iter().any(|s| s.contains("一本")));
    }

    #[test]
    fn test_taekwondo_scoring() {
        let rules = CombatSportsDetailedRules::new();
        let scoring = rules.taekwondo_scoring();
        assert!(!scoring.is_empty());
        assert!(scoring.iter().any(|s| s.contains("躯干")));
    }

    #[test]
    fn test_fencing_rules() {
        let rules = CombatSportsDetailedRules::new();
        let rules_list = rules.fencing_rules();
        assert!(!rules_list.is_empty());
        assert!(rules_list.iter().any(|r| r.contains("剑种")));
    }

    #[test]
    fn test_explain() {
        let rules = CombatSportsDetailedRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("格斗项目详细规则"));
    }
}
