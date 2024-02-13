# transparent_proxy

A very simple proxy that pipes any TCP data data it receives to it's destination

## Usage

```shell
transparent_proxy --bind localhost:4000 --destination localhost:8000
```

Now, all requests to `localhost:4000` will be proxied to `localhost:8000`.

## Testing

Start the destination server...

```shell
deno run -A destination_server.ts
```

...then start the proxy...

```shell
cargo run -- -b localhost:4000 -d localhost:8000
```

...then make some requests to `localhost:4000`:

```shell
curl http://localhost:4000
```

The request body should be printed by the destination server, and the request headers should remain unchanged.

## TODO

- Parse HTTP request and get the intended origin, then connect to that server
