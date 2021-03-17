
CREATE  TABLE `wiser` (
    `id` varchar(50) NOT NULL DEFAULT '' COMMENT '唯一活动码',
    `title` TEXT NOT NULL,
    `body` TEXT NOT NULL
);

INSERT INTO `wiser` VALUES ('1', 'baidu', 'good baidu' ), ('2', 'google', 'The Rust Programming Language');