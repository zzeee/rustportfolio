const path = require('path');
const fs = require('fs');
module.exports = {
  entry: "./index.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
  },
  mode: "development",
  devServer: {
     host: '0.0.0.0',
    https: {
        key: fs.readFileSync('./server.key'),
        cert: fs.readFileSync('./server.crt'),
      //  ca: fs.readFileSync('C:/Users/User/AppData/Local/mkcert/rootCA.pem')
    }
}

};
