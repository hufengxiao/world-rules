//! 球类运动详细规则
//!
//! 本模块提供球类运动的详细规则实现，涵盖足球、篮球、排球、网球、乒乓球、羽毛球等。
//! 符合各国际单项体育联合会标准。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 球类运动类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BallGame {
    /// 足球
    Football,
    /// 篮球
    Basketball,
    /// 排球
    Volleyball,
    /// 网球
    Tennis,
    /// 乒乓球
    TableTennis,
    /// 羽毛球
    Badminton,
    /// 手球
    Handball,
    /// 水球
    WaterPolo,
}

impl BallGame {
    /// 获取运动名称
    pub fn name(&self) -> &'static str {
        match self {
            BallGame::Football => "足球",
            BallGame::Basketball => "篮球",
            BallGame::Volleyball => "排球",
            BallGame::Tennis => "网球",
            BallGame::TableTennis => "乒乓球",
            BallGame::Badminton => "羽毛球",
            BallGame::Handball => "手球",
            BallGame::WaterPolo => "水球",
        }
    }

    /// 是否为奥运会项目
    pub fn is_olympic(&self) -> bool {
        true // 所有项目都是奥运会项目
    }

    /// 是否使用球拍
    pub fn uses_racket(&self) -> bool {
        matches!(
            self,
            BallGame::Tennis | BallGame::TableTennis | BallGame::Badminton
        )
    }

    /// 是否为团体对抗
    pub fn is_team_sport(&self) -> bool {
        matches!(
            self,
            BallGame::Football
                | BallGame::Basketball
                | BallGame::Volleyball
                | BallGame::Handball
                | BallGame::WaterPolo
        )
    }
}

/// 球类运动详细规则
pub struct BallGamesDetailedRules {
    metadata: RuleMetadata,
}

impl BallGamesDetailedRules {
    /// 创建新的球类运动详细规则实例
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("球类运动详细规则", "球类运动详细比赛规则")
                .with_origin("各国际单项体育联合会")
                .with_tags(vec!["体育".into(), "球类".into()]),
        }
    }

    // ==================== 足球规则 ====================

    /// 足球比赛规则
    pub fn football_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛时间: 上下半场各45分钟，中场休息15分钟",
            "场地尺寸: 长90-120米，宽45-90米",
            "球门尺寸: 宽7.32米，高2.44米",
            "队员人数: 每队11人（含门将）",
            "替补名额: 最多5人（正式比赛）",
            "裁判团队: 主裁判、2名边裁、VAR裁判",
            "比赛用球: 周长68-70厘米，重量410-450克",
            "越位规则: 进攻球员在对方半场接球时位置",
        ]
    }

    /// 足球犯规与判罚
    pub fn football_fouls(&self) -> Vec<&'static str> {
        vec![
            "直接任意球: 踢、绊、跳向、冲撞、打、推搡对手",
            "间接任意球: 危险动作、阻挡、守门员持球超时",
            "黄牌: 非体育行为、异议、持续犯规、拖延时间",
            "红牌: 严重犯规、暴力行为、吐痰、两次黄牌",
            "点球: 禁区内犯规（防守方）",
            "越位: 不判罚越位的情况（本方半场、球门球、界外球）",
            "VAR: 只用于进球、点球、红牌、身份错误",
            "视频助理裁判: 主裁判可查看回放",
        ]
    }

    /// 足球比赛时间规则
    pub fn football_timing_rules(&self) -> Vec<&'static str> {
        vec![
            "常规时间: 90分钟（45+45）",
            "补时: 根据比赛实际情况确定",
            "加时赛: 上下半场各15分钟（淘汰赛）",
            "点球大战: 5轮后交替进行",
            "突然死亡: 点球大战5轮后继续",
            "中场休息: 15分钟",
            "加时赛前休息: 5分钟",
            "换人时间: 计入补时",
        ]
    }

    /// 足球越位规则详解
    pub fn football_offside_rules(&self) -> Vec<&'static str> {
        vec![
            "越位位置: 在对方半场，比球和倒数第二名防守球员更接近球门线",
            "越位犯规: 在越位位置参与进攻",
            "不判越位: 球门球、界外球、角球",
            "参与进攻: 触球、干扰对手、获得优势",
            "被动越位: 不参与进攻不判罚",
            "VAR确认: 进球前确认是否越位",
            "画线技术: 使用半自动越位识别系统",
            "越位判罚: 间接任意球（犯规方开球）",
        ]
    }

    // ==================== 篮球规则 ====================

    /// 篮球比赛规则
    pub fn basketball_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛时间: 4节×10分钟（FIBA）、12分钟（NBA）",
            "场地尺寸: 28米×15米（FIBA）、28.65米×15.24米（NBA）",
            "篮筐高度: 3.05米",
            "三分线距离: 6.75米（FIBA）、7.24米（NBA）",
            "队员人数: 每队5人上场，最多12人",
            "裁判: 2名裁判（FIBA）、3名裁判（NBA）",
            "比赛用球: 周长74.9-78厘米，重量567-650克",
            "得分: 2分、3分、罚球1分",
        ]
    }

    /// 篮球时间规则
    pub fn basketball_timing_rules(&self) -> Vec<&'static str> {
        vec![
            "进攻时间: 24秒（NBA）、24秒（FIBA）",
            "8秒规则: 后场8秒内过半场",
            "5秒规则: 持球5秒内需传球或运球",
            "3秒规则: 进攻方在限制区不得超过3秒",
            "罚球时间: 5秒内出手",
            "暂停时长: 1分钟（常规）、20秒（短暂停）",
            "加时赛: 5分钟",
            "节间休息: 2分钟（1-2节、3-4节）、15分钟（中场）",
        ]
    }

    /// 篮球犯规规则
    pub fn basketball_fouls(&self) -> Vec<&'static str> {
        vec![
            "个人犯规: 打手、阻挡、推搡、带球撞人",
            "技术犯规: 非体育行为、延误比赛",
            "恶意犯规: 不必要的过度接触",
            "犯规次数: 个人5次（FIBA）、6次（NBA）离场",
            "全队犯规: 每节超过4次后罚球",
            "罚球: 投篮犯规、技术犯规、全队犯规",
            "自由球: 犯规后掷球入界",
            "挑战: 教练可挑战判罚（NBA）",
        ]
    }

    /// 篮球违例规则
    pub fn basketball_violations(&self) -> Vec<&'static str> {
        vec![
            "走步: 持球移动超过允许步数",
            "二次运球: 停球后再次运球",
            "脚踢球: 故意用脚踢球",
            "拳击球: 用拳击打球",
            "出界: 球或持球者出界",
            "回场: 过半场后回传后场",
            "干扰球: 篮球在篮筐上方时触碰",
            "罚球违例: 罚球时违规进入限制区",
        ]
    }

    // ==================== 排球规则 ====================

    /// 排球比赛规则
    pub fn volleyball_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛形式: 5局3胜制",
            "场地尺寸: 18米×9米",
            "网高: 2.43米（男）、2.24米（女）",
            "队员人数: 每队6人上场",
            "换人: 每局6次换人",
            "比赛用球: 周长65-67厘米，重量260-280克",
            "裁判: 主裁判、副裁判、2名边裁",
            "得分: 每球得分制",
        ]
    }

    /// 排球得分规则
    pub fn volleyball_scoring(&self) -> Vec<&'static str> {
        vec![
            "前4局: 25分，领先2分获胜",
            "第5局: 15分，领先2分获胜",
            "每球得分: 无论发球方，得分即得分",
            "轮转: 得分后轮转位置",
            "发球: 每次得分后换发球方",
            "触球次数: 每侧最多3次触球",
            "拦网: 不计入触球次数",
            "触网: 干扰比赛时判罚",
        ]
    }

    /// 排球技术规则
    pub fn volleyball_techniques(&self) -> Vec<&'static str> {
        vec![
            "发球: 从端线后发出",
            "接发球: 使用垫球、传球",
            "传球: 二传手组织进攻",
            "扣球: 攻击性击球",
            "拦网: 在网前阻挡对方击球",
            "救球: 防守性击球",
            "自由人: 专职防守球员",
            "轮转: 顺时针轮转位置",
        ]
    }

    /// 排球犯规规则
    pub fn volleyball_fouls(&self) -> Vec<&'static str> {
        vec![
            "四次触球: 同侧超过3次触球",
            "连击: 同一人连续触球（拦网除外）",
            "持球: 球在手中停留时间过长",
            "触网: 干扰比赛的触网",
            "过网击球: 在对方空间击球",
            "后排犯规: 后排球员在前区攻击",
            "位置错误: 轮转位置不正确",
            "发球犯规: 发球时踩线",
        ]
    }

    // ==================== 网球规则 ====================

    /// 网球比赛规则
    pub fn tennis_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛形式: 三盘两胜、五盘三胜（男单）",
            "场地尺寸: 长23.77米，宽8.23米（单打）",
            "网高: 中间0.914米，两侧1.07米",
            "比赛用球: 周长6.54-6.86厘米，重量56-59.4克",
            "发球: 每局两人轮换发球",
            "裁判: 主裁判、网裁判、边裁判",
            "挑战系统: 鹰眼系统",
            "热身时间: 5分钟",
        ]
    }

    /// 网球计分规则
    pub fn tennis_scoring(&self) -> Vec<&'static str> {
        vec![
            "分值: 0（Love）、15、30、40、局点",
            "平分: 40-40时需连续2分获胜",
            "占先: 平分后领先1分",
            "局: 每局得分最高者获胜",
            "盘: 先赢6局者获胜（需领先2局）",
            "抢七: 6-6时进行抢七局",
            "抢七局: 先到7分者获胜（需领先2分）",
            "决胜盘: 6-6时进行长盘或抢十",
        ]
    }

    /// 网球发球规则
    pub fn tennis_serve_rules(&self) -> Vec<&'static str> {
        vec![
            "发球位置: 站在底线后方",
            "发球区: 交替发向左右两个区域",
            "脚误: 发球时踩线",
            "第一发球: 失误后可发第二球",
            "双误: 两次发球失误，失分",
            "发球触网: 重新发球（LET）",
            "发球时间: 每分20秒内发出",
            "发球次序: 每局后轮换",
        ]
    }

    /// 网球场地类型
    pub fn tennis_court_types(&self) -> Vec<&'static str> {
        vec![
            "硬地: 中速球场（澳网、美网）",
            "红土: 慢速球场（法网）",
            "草地: 快速球场（温网）",
            "室内: 硬地或地毯",
            "球速特性: 影响球的弹跳和速度",
            "比赛特点: 不同场地技术要求不同",
            "场地维护: 不同场地维护方式不同",
            "赛季安排: 根据场地安排赛季",
        ]
    }

    // ==================== 乒乓球规则 ====================

    /// 乒乓球比赛规则
    pub fn table_tennis_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛形式: 7局4胜制",
            "每局分数: 11分制",
            "场地尺寸: 2.74米×1.525米",
            "球网高度: 15.25厘米",
            "比赛用球: 直径40毫米，重量2.7克（白色/橙色）",
            "球拍: 大小形状不限，一面红色一面黑色",
            "发球: 每2分轮换发球",
            "裁判: 主裁判、副裁判",
        ]
    }

    /// 乒乓球得分规则
    pub fn table_tennis_scoring(&self) -> Vec<&'static str> {
        vec![
            "每局11分: 领先2分获胜",
            "每2分轮换发球",
            "10-10后: 每分轮换发球",
            "触网: 发球触网重发（有效触网）",
            "擦边: 球擦台面边缘为有效球",
            "得分条件: 对方失误",
            "失误: 未击球、触网未过、出界",
            "比赛暂停: 每人可暂停1分钟",
        ]
    }

    /// 乒乓球技术规则
    pub fn table_tennis_techniques(&self) -> Vec<&'static str> {
        vec![
            "发球: 抛球至少16厘米",
            "正手攻球: 正手位进攻",
            "反手攻球: 反手位进攻",
            "削球: 防守性击球",
            "弧圈球: 上旋进攻",
            "快攻: 快速击球",
            "摆短: 短球技术",
            "拧拉: 台内进攻",
        ]
    }

    // ==================== 羽毛球规则 ====================

    /// 羽毛球比赛规则
    pub fn badminton_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛形式: 3局2胜制",
            "每局分数: 21分制",
            "场地尺寸: 13.4米×5.18米（单打）、6.1米（双打）",
            "网高: 1.55米",
            "比赛用球: 羽毛球（16根羽毛）",
            "球拍: 长度不超过68厘米",
            "发球: 每分轮换发球",
            "裁判: 主裁判、发球裁判、边裁",
        ]
    }

    /// 羽毛球得分规则
    pub fn badminton_scoring(&self) -> Vec<&'static str> {
        vec![
            "每局21分: 领先2分获胜",
            "20-20后: 领先2分获胜",
            "29-29后: 先到30分获胜",
            "每分轮换发球",
            "发球方得分: 继续发球",
            "接发球方得分: 换发球",
            "得分条件: 球落地在对方场地",
            "失误: 球出界、触网未过、未击球",
        ]
    }

    /// 羽毛球技术规则
    pub fn badminton_techniques(&self) -> Vec<&'static str> {
        vec![
            "发球: 短球、长球",
            "高远球: 高远球到后场",
            "杀球: 进攻性下压球",
            "吊球: 轻吊网前",
            "网前球: 网前小球",
            "挑球: 防守性高球",
            "平抽球: 中场平击",
            "扑球: 网前扑杀",
        ]
    }

    /// 羽毛球比赛项目
    pub fn badminton_events(&self) -> Vec<&'static str> {
        vec![
            "男子单打",
            "女子单打",
            "男子双打",
            "女子双打",
            "混合双打",
            "团体赛: 苏迪曼杯、汤姆斯杯、尤伯杯",
            "奥运会: 5个单项",
            "世锦赛: 5个单项",
        ]
    }

    // ==================== 通用规则 ====================

    /// 比赛装备要求
    pub fn equipment_requirements(&self) -> Vec<&'static str> {
        vec![
            "足球: 足球鞋、护腿板",
            "篮球: 篮球鞋、运动服",
            "排球: 运动服、运动鞋",
            "网球: 网球拍、网球鞋",
            "乒乓球: 乒乓球拍、运动服",
            "羽毛球: 羽毛球拍、运动鞋",
            "护具: 根据项目要求佩戴",
            "服装: 符合比赛规定",
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
            "兴奋剂控制: 比赛期间检测",
        ]
    }

    /// 比赛暂停与中断
    pub fn interruption_rules(&self) -> Vec<&'static str> {
        vec![
            "天气中断: 户外项目因天气暂停",
            "伤病暂停: 医疗暂停",
            "技术故障: 设备故障暂停",
            "观众干扰: 严重干扰时暂停",
            "暂停后恢复: 从暂停点继续",
            "取消比赛: 无法继续时取消",
            "重赛: 特殊情况重赛",
            "时间限制: 各项目规定不同",
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
            "青少年规则: 部分规则有调整",
            "大师赛: 35岁以上",
        ]
    }
}

impl Default for BallGamesDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BallGamesDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("ball_games_detailed")
    }

    fn explain(&self) -> String {
        format!(
            "【球类运动详细规则】\n\
            涵盖足球、篮球、排球、网球、乒乓球、羽毛球等球类运动\n\n\
            足球规则:\n{}\n\n\
            篮球规则:\n{}\n\n\
            排球规则:\n{}\n\n\
            网球规则:\n{}\n\n\
            乒乓球规则:\n{}\n",
            self.football_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.basketball_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.volleyball_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.tennis_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.table_tennis_rules()
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
    fn test_ball_games_detailed_rules_creation() {
        let rules = BallGamesDetailedRules::new();
        assert_eq!(rules.metadata().name, "球类运动详细规则");
    }

    #[test]
    fn test_ball_game_properties() {
        assert!(BallGame::Football.is_olympic());
        assert!(BallGame::Football.is_team_sport());
        assert!(!BallGame::Tennis.is_team_sport());
        assert!(BallGame::Tennis.uses_racket());
        assert!(!BallGame::Basketball.uses_racket());
    }

    #[test]
    fn test_football_rules() {
        let rules = BallGamesDetailedRules::new();
        let rules_list = rules.football_rules();
        assert!(!rules_list.is_empty());
        assert!(rules_list.iter().any(|r| r.contains("比赛时间")));
    }

    #[test]
    fn test_basketball_rules() {
        let rules = BallGamesDetailedRules::new();
        let rules_list = rules.basketball_rules();
        assert!(!rules_list.is_empty());
        assert!(rules_list.iter().any(|r| r.contains("比赛时间")));
    }

    #[test]
    fn test_volleyball_scoring() {
        let rules = BallGamesDetailedRules::new();
        let scoring = rules.volleyball_scoring();
        assert!(!scoring.is_empty());
        assert!(scoring.iter().any(|s| s.contains("25分")));
    }

    #[test]
    fn test_tennis_scoring() {
        let rules = BallGamesDetailedRules::new();
        let scoring = rules.tennis_scoring();
        assert!(!scoring.is_empty());
        assert!(scoring.iter().any(|s| s.contains("40")));
    }

    #[test]
    fn test_badminton_events() {
        let rules = BallGamesDetailedRules::new();
        let events = rules.badminton_events();
        assert!(!events.is_empty());
        assert!(events.iter().any(|e| e.contains("单打")));
    }

    #[test]
    fn test_explain() {
        let rules = BallGamesDetailedRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("球类运动详细规则"));
    }
}