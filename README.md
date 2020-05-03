### Dev

```
RUST_BACKTRACE=1 RUST_LOG=debug cargo run
```

```
docker run --rm --name haproxy-spoa-example -v $(pwd)/haproxy_conf:/usr/local/etc/haproxy --network host haproxy:2.2-rc-alpine haproxy -f /usr/local/etc/haproxy/haproxy.cfg -d -V
```

```
curl http://127.0.0.1:6002/ -o /dev/null -v
```
