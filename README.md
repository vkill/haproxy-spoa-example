### Dev

```
RUST_BACKTRACE=1 RUST_LOG=debug cargo run
```

```
docker run --rm --name haproxy-spoa-example -v $(pwd)/haproxy_conf:/usr/local/etc/haproxy --network host haproxy:2.1-alpine haproxy -f /usr/local/etc/haproxy/haproxy.cfg -d -V
```

```
open http://127.0.0.1:6002/
```
