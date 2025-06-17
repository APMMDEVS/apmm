use pyo3::prelude::*;

mod cmds;
use cmds::ApmmConfig as CoreConfig;


/// CLI 入口函数 - 使用共享的命令处理逻辑
#[pyfunction]
fn cli() -> PyResult<()> {
    // 获取命令行参数 - 简化版本，只显示帮助
    cmds::show_help();
    Ok(())
}

/// Python 模块定义
#[pymodule]
fn apmmcore(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(cli, m)?)?;
    Ok(())
}