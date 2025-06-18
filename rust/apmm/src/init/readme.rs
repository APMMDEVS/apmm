/// 为 Magisk 模块生成 README.md 模板
pub fn generate_readme_md(module_id: &str, author: &str) -> String {    format!(r#"# {}

使用 APMM (Android Package Module Manager) 创建的 APM(Android Patch Module)

## 描述

love from APMM ❤️

## 功能特性

请在此处描述您的模块功能

## 使用方法

请在此处描述如何使用您的模块

## 开发贡献

此模块使用 APMM 创建。要贡献代码：

1. 克隆此仓库
2. 进行修改
3. 在不同设备上充分测试
4. 提交 Pull Request

### 构建

```bash
# 构建模块
apmm build

# 同步 && 刷新
apmm sync

# 版本控制
apmm sync -U 
(更新模块版本）

# 运行组合操作(module.prop定义内容)
apmm run <script_name>
```

## 许可证

本项目使用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

## 致谢

- 感谢 Magisk/Kernelsu/Apatch
- 感谢 [APMM](https://github.com/APMMDEVS/apmm) 团队提供的工具和支持

## 支持

如果您遇到任何问题或有疑问：
- 在 [GitHub Issues](https://github.com/APMMDEVS/apmm/issues) 中提交问题
- 加入我们的社区讨论

## 免责声明

APMM 不对生成的模块负责

## 作者

作者: {}

## 参考链接

- [KERNELSU 模块开发指南](https://kernelsu.org/zh_CN/guide/module.html)
- [Magisk 模块开发指南](https://topjohnwu.github.io/Magisk/guides.html)
- [APMM 文档](https://github.com/apmm/apmm)
"#, module_id, author)
}
