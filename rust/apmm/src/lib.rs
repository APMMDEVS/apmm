use pyo3::prelude::*;

mod cmds;
use cmds::ApmmConfig as CoreConfig;

/// APMM 配置解析器 - Python 包装器
#[pyclass]
struct ApmmConfig {
    #[pyo3(get)]
    id: String,
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    description: String,
    #[pyo3(get)]
    version: String,
    #[pyo3(get)]
    version_code: i64,
    #[pyo3(get)]
    author: String,
    #[pyo3(get)]
    license: String,
}

impl From<CoreConfig> for ApmmConfig {
    fn from(config: CoreConfig) -> Self {
        ApmmConfig {
            id: config.id,
            name: config.name,
            description: config.description,
            version: config.version,
            version_code: config.version_code,
            author: config.author,
            license: config.license,
        }
    }
}

#[pymethods]
impl ApmmConfig {
    #[new]
    fn new(id: String, name: String, description: String, version: String, version_code: i64, author: String, license: String) -> Self {
        ApmmConfig {
            id,
            name,
            description,
            version,
            version_code,
            author,
            license,
        }
    }

    fn __repr__(&self) -> String {
        format!("ApmmConfig(id='{}', name='{}', version='{}')", self.id, self.name, self.version)
    }
}

/// 解析 module.prop 文件
#[pyfunction]
fn parse_module_prop(content: &str) -> PyResult<ApmmConfig> {
    let core_config = CoreConfig::from_content(content)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e))?;
    Ok(ApmmConfig::from(core_config))
}

/// 构建模块
#[pyfunction]
fn build_module() -> PyResult<String> {
    let result = cmds::cmd_build()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))?;
    Ok(result)
}

/// 安装模块
#[pyfunction]
fn install_module() -> PyResult<String> {
    let result = cmds::cmd_install()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))?;
    Ok(result)
}

/// 移除模块
#[pyfunction]
fn remove_module() -> PyResult<String> {
    let result = cmds::cmd_remove()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))?;
    Ok(result)
}

/// 显示模块信息
#[pyfunction]
fn info_module() -> PyResult<String> {
    let result = cmds::cmd_info()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))?;
    Ok(result)
}

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
    m.add_function(wrap_pyfunction!(parse_module_prop, m)?)?;
    m.add_function(wrap_pyfunction!(build_module, m)?)?;
    m.add_function(wrap_pyfunction!(install_module, m)?)?;
    m.add_function(wrap_pyfunction!(remove_module, m)?)?;
    m.add_function(wrap_pyfunction!(info_module, m)?)?;
    m.add_class::<ApmmConfig>()?;
    Ok(())
}