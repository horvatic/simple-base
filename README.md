# Simple Base
A very simple document database. This is just a demo. This is not feature complete, and will never be feature complete.

## Commands:
- insert: Insert Data

example: `{ "command" : "insert", "data" : ["hello", "all", "12345" ] }`

- where: Fetch Data

example: `{ "command" : "where", "id": "f3659469-164f-4464-a423-f49aa53ed677" }`

- delete: Delete Data

example: `{ "command" : "delete", "id" : "f3659469-164f-4464-a423-f49aa53ed677" }`

- update: Not working

example: 
## Testing
curl -v telnet://127.0.0.1:8080
```
{ "command" : "insert", "data" : ["hello", "all", "12345" ] }
{"result":"id f3659469-164f-4464-a423-f49aa53ed677"}

{ "command" : "where", "id": "f3659469-164f-4464-a423-f49aa53ed677" }
["hello","all","12345"]

{ "command" : "delete", "id" : "f3659469-164f-4464-a423-f49aa53ed677" }        
{"result":"record deleted"}

{ "command" : "update" }
{"result":"can not run update. Please use where, delete, insert"}
```