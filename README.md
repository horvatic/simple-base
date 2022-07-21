# Simple Base
A very simple document database. This is just a demo. This is not feature complete, and will never be feature complete.

## Commands:
- insert: Insert Data

example: `{ "command" : "insert", "data" : ["hello", "all", "12345" ] }`

- where: Fetch Data

example: `{ "command" : "where", "id": "f3659469-164f-4464-a423-f49aa53ed677" }`

- delete: Delete Data

example: `{ "command" : "delete", "id" : "f3659469-164f-4464-a423-f49aa53ed677" }`

- update: Update Data

example: `{ "command" : "update", "id" : "fcd89674-690a-4ca9-b164-e1acabddfdd3", "data" : ["goodbye", "tttt", "234" ] }`

- exit: Close Session

example: `{ "command" : "exit" }`

## Testing
curl -v telnet://127.0.0.1:8080
```
{ "command" : "insert", "data" : ["hello", "all", "12345" ] }
{"result":"id f3659469-164f-4464-a423-f49aa53ed677"}

{ "command" : "where", "id": "f3659469-164f-4464-a423-f49aa53ed677" }
["hello","all","12345"]

{ "command" : "delete", "id" : "f3659469-164f-4464-a423-f49aa53ed677" }        
{"result":"record deleted"}

{ "command" : "update", "id" : "fcd89674-690a-4ca9-b164-e1acabddfdd3", "data" : ["goodbye", "tttt", "234" ] }
{"result":"id fcd89674-690a-4ca9-b164-e1acabddfdd3"}

{ "command" : "exit" }
```