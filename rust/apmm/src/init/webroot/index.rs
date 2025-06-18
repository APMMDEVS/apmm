/// 生成符合 KernelSU WebUI 规范的 index.html 文件
pub fn generate_index_html(module_id: &str) -> String {
    format!(r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} 模块控制面板</title>
    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}

        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            padding: 20px;
            color: #333;
        }}

        .container {{
            max-width: 800px;
            margin: 0 auto;
            background: rgba(255, 255, 255, 0.95);
            border-radius: 15px;
            box-shadow: 0 20px 40px rgba(0, 0, 0, 0.1);
            overflow: hidden;
            backdrop-filter: blur(10px);
        }}

        .header {{
            background: linear-gradient(135deg, #4a9eff 0%, #3d8bfd 100%);
            color: white;
            padding: 30px;
            text-align: center;
        }}

        .header h1 {{
            font-size: 2rem;
            margin-bottom: 10px;
            font-weight: 300;
        }}

        .header p {{
            opacity: 0.9;
        }}

        .content {{
            padding: 30px;
        }}

        .section {{
            margin-bottom: 30px;
        }}

        .section h2 {{
            color: #4a9eff;
            font-size: 1.3rem;
            margin-bottom: 15px;
            border-bottom: 2px solid #4a9eff;
            padding-bottom: 8px;
        }}

        .status-card {{
            display: flex;
            align-items: center;
            gap: 15px;
            padding: 20px;
            background: #f8f9fa;
            border-radius: 10px;
            border: 1px solid #e9ecef;
            margin-bottom: 20px;
        }}

        .status-indicator {{
            width: 20px;
            height: 20px;
            border-radius: 50%;
            background: #28a745;
            box-shadow: 0 0 0 3px rgba(40, 167, 69, 0.3);
            animation: pulse 2s infinite;
        }}

        @keyframes pulse {{
            0% {{ box-shadow: 0 0 0 0 rgba(40, 167, 69, 0.7); }}
            70% {{ box-shadow: 0 0 0 10px rgba(40, 167, 69, 0); }}
            100% {{ box-shadow: 0 0 0 0 rgba(40, 167, 69, 0); }}
        }}

        .info-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 15px;
        }}

        .info-item {{
            padding: 15px;
            background: #f8f9fa;
            border-radius: 8px;
            border: 1px solid #e9ecef;
        }}

        .info-item strong {{
            color: #495057;
            display: block;
            margin-bottom: 5px;
        }}

        .info-item span {{
            color: #6c757d;
            font-family: 'Courier New', monospace;
        }}

        .actions {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 15px;
        }}

        .btn {{
            padding: 12px 20px;
            background: linear-gradient(135deg, #4a9eff 0%, #3d8bfd 100%);
            color: white;
            border: none;
            border-radius: 8px;
            cursor: pointer;
            transition: all 0.3s ease;
            font-size: 0.9rem;
        }}

        .btn:hover {{
            transform: translateY(-2px);
            box-shadow: 0 8px 25px rgba(74, 158, 255, 0.4);
        }}

        .btn:active {{
            transform: translateY(0);
        }}

        .log-output {{
            background: #2d3748;
            color: #e2e8f0;
            padding: 15px;
            border-radius: 8px;
            font-family: 'Courier New', monospace;
            font-size: 0.9rem;
            max-height: 200px;
            overflow-y: auto;
            white-space: pre-wrap;
            margin-top: 15px;
            display: none;
        }}

        .footer {{
            background: #f8f9fa;
            padding: 20px;
            text-align: center;
            color: #6c757d;
            border-top: 1px solid #e9ecef;
        }}

        @media (max-width: 768px) {{
            body {{ padding: 10px; }}
            .header {{ padding: 20px; }}
            .content {{ padding: 20px; }}
            .actions {{ grid-template-columns: 1fr; }}
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>{} 模块</h1>
            <p>KernelSU 模块控制面板</p>
        </div>
        
        <div class="content">
            <div class="section">
                <h2>📊 模块状态</h2>
                <div class="status-card">
                    <div class="status-indicator" id="statusIndicator"></div>
                    <span id="statusText">模块运行中</span>
                </div>
            </div>
            
            <div class="section">
                <h2>ℹ️ 模块信息</h2>
                <div class="info-grid">
                    <div class="info-item">
                        <strong>模块ID:</strong>
                        <span>{}</span>
                    </div>
                    <div class="info-item">
                        <strong>版本:</strong>
                        <span>v0.1.0</span>
                    </div>
                    <div class="info-item">
                        <strong>作者:</strong>
                        <span>APMM Team</span>
                    </div>
                    <div class="info-item">
                        <strong>创建工具:</strong>
                        <span>APMM</span>
                    </div>
                </div>
            </div>
            
            <div class="section">
                <h2>🔧 操作</h2>
                <div class="actions">
                    <button class="btn" onclick="refreshStatus()">🔄 刷新状态</button>
                    <button class="btn" onclick="viewSystemInfo()">📋 系统信息</button>
                    <button class="btn" onclick="runTest()">🧪 运行测试</button>
                    <button class="btn" onclick="toggleFullScreen()">🖥️ 全屏切换</button>
                </div>
                <div class="log-output" id="logOutput"></div>
            </div>
        </div>
        
        <div class="footer">
            <p>&copy; 2024 APMM Team. 使用 Android Package Module Manager 创建</p>
        </div>
    </div>

    <script type="module">
        // 导入 KernelSU API
        import {{ exec, toast, fullScreen }} from 'kernelsu';

        // 全局变量
        let isFullScreen = false;

        // 刷新状态
        window.refreshStatus = async function() {{
            try {{
                const statusIndicator = document.getElementById('statusIndicator');
                const statusText = document.getElementById('statusText');
                
                // 检查模块状态
                const {{ errno, stdout }} = await exec('ls /data/adb/modules/{}');
                
                if (errno === 0) {{
                    statusIndicator.style.background = '#28a745';
                    statusText.textContent = '模块运行中';
                    toast('状态刷新成功');
                }} else {{
                    statusIndicator.style.background = '#dc3545';
                    statusText.textContent = '模块未运行';
                    toast('模块状态异常');
                }}
            }} catch (error) {{
                console.error('刷新状态失败:', error);
                toast('刷新状态失败');
            }}
        }};

        // 查看系统信息
        window.viewSystemInfo = async function() {{
            try {{
                const logOutput = document.getElementById('logOutput');
                logOutput.style.display = 'block';
                
                const commands = [
                    'uname -a',
                    'getprop ro.product.model',
                    'getprop ro.build.version.release',
                    'df -h /data'
                ];
                
                let output = '=== 系统信息 ===\\n';
                
                for (const cmd of commands) {{
                    const {{ errno, stdout }} = await exec(cmd);
                    if (errno === 0) {{
                        output += `$ ${{cmd}}\\n${{stdout}}\\n\\n`;
                    }}
                }}
                
                logOutput.textContent = output;
                toast('系统信息获取成功');
            }} catch (error) {{
                console.error('获取系统信息失败:', error);
                toast('获取系统信息失败');
            }}
        }};

        // 运行测试
        window.runTest = async function() {{
            try {{
                const logOutput = document.getElementById('logOutput');
                logOutput.style.display = 'block';
                
                let output = '=== {} 模块测试 ===\\n';
                output += `开始时间: ${{new Date().toLocaleString()}}\\n\\n`;
                
                // 测试模块目录
                const {{ errno: dirErrno, stdout: dirStdout }} = await exec('ls -la /data/adb/modules/{}');
                if (dirErrno === 0) {{
                    output += '✅ 模块目录存在\\n';
                    output += `目录内容:\\n${{dirStdout}}\\n\\n`;
                }} else {{
                    output += '❌ 模块目录不存在\\n\\n';
                }}
                
                // 测试模块属性
                const {{ errno: propErrno, stdout: propStdout }} = await exec('cat /data/adb/modules/{}/module.prop');
                if (propErrno === 0) {{
                    output += '✅ 模块属性文件存在\\n';
                    output += `属性内容:\\n${{propStdout}}\\n\\n`;
                }} else {{
                    output += '❌ 模块属性文件不存在\\n\\n';
                }}
                
                output += `完成时间: ${{new Date().toLocaleString()}}\\n`;
                
                logOutput.textContent = output;
                toast('测试完成');
            }} catch (error) {{
                console.error('运行测试失败:', error);
                toast('运行测试失败');
            }}
        }};

        // 切换全屏
        window.toggleFullScreen = function() {{
            try {{
                isFullScreen = !isFullScreen;
                fullScreen(isFullScreen);
                toast(isFullScreen ? '已进入全屏' : '已退出全屏');
            }} catch (error) {{
                console.error('切换全屏失败:', error);
                toast('切换全屏失败');
            }}
        }};        // 页面加载完成后初始化
        document.addEventListener('DOMContentLoaded', function() {{
            console.log('{} 模块 WebUI 已加载');
            toast('欢迎使用 {} 模块');
        }});
    </script>
</body>
</html>"#, module_id, module_id, module_id, module_id, module_id, module_id, module_id, module_id, module_id)
}
