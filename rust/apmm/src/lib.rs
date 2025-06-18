use pyo3::prelude::*;

// 声明模块
mod cmds;
mod env;
mod init;
mod build;
mod core;
mod run;
mod sync;


/// CLI 入口函数 - 使用共享的命令处理逻辑
#[pyfunction]
#[pyo3(signature = (args=None))]
fn cli(args: Option<Vec<String>>) -> PyResult<()> {
    // 获取命令行参数
    let command_args = if let Some(args) = args {
        args
    } else {
        // 如果没有提供参数，从 sys.argv 获取并排除第一个参数（程序路径）
        Python::with_gil(|py| {
            let sys = py.import("sys")?;
            let argv: Vec<String> = sys.getattr("argv")?.extract()?;
            // 排除第一个参数（程序路径）
            Ok::<Vec<String>, PyErr>(argv.into_iter().skip(1).collect())
        })?
    };
    
    // 使用共享的命令处理逻辑
    match cmds::handle_command(&command_args) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            Err(pyo3::exceptions::PyRuntimeError::new_err(e))
        }
    }
}

/// Python 模块定义
#[pymodule]
fn apmmcore(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(cli, m)?)?;
    Ok(())
}