#!/usr/bin/env bash
set -e

BASE="http://127.0.0.1:24433/mcp"

call() {
  RESP=$(curl -s -X POST "$BASE" \
    -H "Accept: application/json, text/event-stream" \
    -H "Content-Type: application/json" \
    -H "mcp-session-id: $SESSION" \
    -d "$1")
  echo "$RESP"
  if echo "$RESP" | grep -q '"error"'; then
    echo "ERROR: request failed" >&2
    exit 1
  fi
}

# Initialize and capture session ID
echo "Initialize"
INIT_RESP=$(curl -s -D - -X POST "$BASE" \
  -H "Accept: application/json, text/event-stream" \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"0.1"}}}')

SESSION=$(echo "$INIT_RESP" | grep -i "mcp-session-id" | awk '{print $2}' | tr -d '\r')
echo "Session: $SESSION"

echo ""
echo "Initialized notification"
call '{"jsonrpc":"2.0","method":"notifications/initialized","params":{}}'

echo ""
echo "Add objective"
call '{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"add_objective","arguments":{"title":"work","context":"Work related tasks","priority":4}}}'

echo ""
echo "Add task"
call '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"add_task","arguments":{"title":"Write tests","context":"Write unit tests for jebbysays","priority":4,"tags":["rust","testing"]}}}'

echo ""
echo "List incomplete tasks"
call '{"jsonrpc":"2.0","id":4,"method":"resources/read","params":{"uri":"tasks://incomplete"}}'

echo ""
echo "List all objectives"
call '{"jsonrpc":"2.0","id":5,"method":"resources/read","params":{"uri":"objectives://all"}}'

echo ""
echo "List prompts"
call '{"jsonrpc":"2.0","id":6,"method":"prompts/list","params":{}}'

echo ""
echo "Get briefing prompt (no filter)"
call '{"jsonrpc":"2.0","id":7,"method":"prompts/get","params":{"name":"briefing","arguments":{}}}'

echo ""
echo "Get briefing prompt (filtered to work)"
call '{"jsonrpc":"2.0","id":8,"method":"prompts/get","params":{"name":"briefing","arguments":{"objective":"work"}}}'

echo ""
echo "Get triage prompt"
call '{"jsonrpc":"2.0","id":9,"method":"prompts/get","params":{"name":"triage","arguments":{}}}'

echo ""
echo "Get retro prompt (default 7 days)"
call '{"jsonrpc":"2.0","id":10,"method":"prompts/get","params":{"name":"retro","arguments":{}}}'

echo ""
echo "Get retro prompt (last 30 days)"
call '{"jsonrpc":"2.0","id":11,"method":"prompts/get","params":{"name":"retro","arguments":{"days":30}}}'

echo ""
echo "Read completed tasks last 7 days"
call '{"jsonrpc":"2.0","id":12,"method":"resources/read","params":{"uri":"tasks://completed/7"}}'
