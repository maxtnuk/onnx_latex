import traceback
import sys

import asyncio
import aiohttp

async def fetch(session):
    files ={"model": open("../examples/latex_test/test_models/l2s.onnx","rb")}
    async with session.post('http://127.0.0.1:1234/parse_model', data=files) as resp:
        print(resp.status)
        recv = await resp.read()
        print(recv)

async def go():
    async with aiohttp.ClientSession() as session:
        await fetch(session)

loop = asyncio.get_event_loop()
loop.run_until_complete(go())