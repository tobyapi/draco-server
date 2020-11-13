# draco-server

## Usage
```sh
$ git clone https://github.com/tobyapi/draco-server
$ cd draco-server
$ docker build -t draco-server .
$ docker run -p 8000:8000  draco-server
$ curl -X POST -F upfile=@<OBJ or PLY file> http://localhost:8000
```