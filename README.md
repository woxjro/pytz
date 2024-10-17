# pytz: Compiler from Python to MLIR Michelson dialect

```sh
$ python3 -m venv venv
$ source venv/bin/activate
$ pip3 install -r ./examples/python/requirements.txt
```

## type checking

```sh
$ mypy ./examples/python/<file>.py
```

## Compile Python to MLIR

```sh
$ cargo run -- --input ./examples/python/boomerang.py --output ./boomerang.mlir
```
