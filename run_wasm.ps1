# prereqs: "npm install simplehttpserver -g"

cd www
Start-Process "simplehttpserver.cmd" 
Start-Process http://localhost:8000
cd ..
