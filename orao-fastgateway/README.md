# ORAO - Fast Gateway Oracle  
Orao is a fast and data agnostic Oracle suitable for any blockchain.  
It's server module wating for connection on port tcp/8000  

## Pre-Requirements
- Install [Rust language](https://www.rust-lang.org)  
- Install [Postgresql Database](https://www.postgresql.org)  
- Launch orao-server

## How to build
```sh
cargo build
```
## How to run

You must set an environment variable ORAODB to set the database connection string:  
```
postgres://orao:orao4oraoandrey@db:5432/orao
```
for example in Linux: 
```bash
export ORAODB=postgres://oraodbuser:dbpassword@host:5432/oraodbname
```

run with cargo:    
```sh
cargo run
```
or  

```sh
./target/relase/orao-fast-gateway  
```

## Examples

You can send data to the Oracle from a Data Provider using "curl":  
```bash
curl 'http://localhost:8000/v2/update' -X POST -d '{"data":[{"value_id":[22,2,3,4,5,6,78],"vector_id":2000002,"protocol_id":211,"value":12.2},{"value_id":[22,2,3,4,5,6,78],"vector_id":2000003,"protocol_id":211,"value":15.1}],"provider":[18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18]}' -H "Content-Type: application/json"
```
```bash
curl 'http://localhost:8000/v2/update' -X POST -d '{"data":[{"value_id":[22,2,3,4,5,6,78],"vector_id":2000002,"protocol_id":211,"value":15.1},{"value_id":[22,2,3,4,5,6,78],"vector_id":2000003,"protocol_id":211,"value":19.7}],"provider":[18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18]}' -H "Content-Type: application/json"
```
```bash
curl 'http://localhost:8000/v2/update' -X POST -d '{"data":[{"value_id":[22,2,3,4,5,6,78],"vector_id":2000002,"protocol_id":211,"value":21.2},{"value_id":[22,2,3,4,5,6,78],"vector_id":2000003,"protocol_id":211,"value":8.5}],"provider":[18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18]}' -H "Content-Type: application/json"
```

You can query the Average of the data submitted above:  
```bash
curl 'http://localhost:8000/v2/average' -X POST -d '{"vector_id":2000002}' -H "Content-Type: application/json"
```
```bash
curl 'http://localhost:8000/v2/average' -X POST -d '{"vector_id":2000003}' -H "Content-Type: application/json"
```
You can query the Average. Minimum, Maximum and standard Deviation of the data submitted above:  

```bash
curl 'http://localhost:8000/v2/stats' -X POST -d '{"vector_id":2000002}' -H "Content-Type: application/json"
```

```bash
curl 'http://localhost:8000/v2/stats' -X POST -d '{"vector_id":2000003}' -H "Content-Type: application/json"
```



