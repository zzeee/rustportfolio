const Koa = require('koa');
const app = new Koa();
const http = require('http');

const swagger = require("swagger2");
const { ui, validate } = require("swagger2-koa");
const swaggerDocument = swagger.loadDocumentSync("swagger.yaml");

const mode=process.env.MODE
let port=4000
console.log('Started in mode:',mode,` port:${port}`)
if (mode==='debug') {
    console.log('Swagger connected')
    app.use(ui(swaggerDocument, "/swagger"))
}
http.createServer(app.callback()).listen(port);
