# APMCP (Android Patch Module Mcp server) 

import asyncio
import os
from pathlib import Path
from typing import Self

from mcp.server.fastmcp import FastMCP

class ApmcpMeta(type):
    """
    Apmcp的元类
    """
    _root :Path | None = None
    _meta :Path | None = None
    _cache :Path | None = None
    _tmp :Path | None = None
    _data :Path | None = None
    @property
    def root(self) -> Path:
        if self._root is None:
            self._root = Path(os.getenv('APMCP_ROOT', Path().home() / "data" / "adb" / "apmcp" ))
        if not self._root.exists():
            self._root.mkdir(parents=True, exist_ok=True)
        return self._root
    
    @property
    def meta(self) -> Path:
        if self._meta is None:
            self._meta = self.root / "meta.toml"
        if not self._meta.exists():
            self._meta.parent.mkdir(parents=True, exist_ok=True)
            self._meta.touch()
        return self._meta

    @property
    def cache(self) -> Path:
        if self._cache is None:
            self._cache = self.root / "cache"
        if not self._cache.exists():
            self._cache.mkdir(parents=True, exist_ok=True)
        return self._cache
    
    @property
    def tmp(self) -> Path:
        if self._tmp is None:
            self._tmp = self.root / "tmp"
        if not self._tmp.exists():
            self._tmp.mkdir(parents=True, exist_ok=True)
        return self._tmp
    
    @property
    def data(self) -> Path:
        if self._data is None:
            self._data = self.root / "data"
        if not self._data.exists():
            self._data.mkdir(parents=True, exist_ok=True)
        return self._data
    
class ApMcp(FastMCP,metaclass=ApmcpMeta):
    """
    APMCP (Android Patch Module Mcp server)
    
    """
    _mcp = None
    @classmethod
    def getMcp(cls,host: str = "localhost",port: int = 10240, new: bool = False ):
        """
        获取MCP实例
        """
        if cls._mcp is None or new:
            cls._mcp = cls(host=host, port=port)
            cls._mcp = cls.init(cls._mcp)
        return cls._mcp

    @classmethod
    def stdio(cls):
        """
        以stdio方式运行MCP服务器
        """
        mcp = cls.getMcp()
        asyncio.run(mcp.run_stdio_async())

    @classmethod
    def sse(cls, host: str = "localhost", port: int = 10240):
        """
        以SSE方式运行MCP服务器
        """
        mcp = cls.getMcp(host=host, port=port, new=True)
        asyncio.run(mcp.run_sse_async())


    @classmethod
    def init(cls,mcp: Self) ->  Self:
        """
        MCP初始化方法
        这样是延迟加载
        """
        @mcp.tool()
        def hello(): # type: ignore
            return "Hello, APMCP!"

        return mcp

def __getattr__(item: str):
        """
        获取MCP实例的属性
        """
        if item == "mcp":
            return ApMcp.getMcp()
        raise AttributeError(f"寄！")

from click import option, group

@group()
def cli():
    pass

@cli.command()
@option('--host', default='localhost', help='MCP server host')
@option('--port', default=10240, help='MCP server port')
def stdio(host: str, port: int):
    """
    以stdio方式运行MCP服务器
    """
    ApMcp.stdio()

@cli.command()
@option('--host', default='localhost', help='MCP server host')
@option('--port', default=10240, help='MCP server port')
def sse(host: str, port: int):
    """
    以SSE方式运行MCP服务器
    """
    ApMcp.sse(host=host, port=port)





if __name__ == "__main__":
    cli()