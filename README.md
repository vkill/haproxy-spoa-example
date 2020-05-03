### Dev

```
RUST_BACKTRACE=1 RUST_LOG=debug cargo run
```

```
docker run --rm --name haproxy-spoa-example -v $(pwd)/haproxy_conf:/usr/local/etc/haproxy -v $(pwd)/haproxy_run:/var/run -e FE_BIND=unix@/var/run/haproxy.sock --network host haproxy:2.2-rc-alpine haproxy -f /usr/local/etc/haproxy/haproxy.cfg -d -V
```

```
curl --unix-socket $(pwd)/haproxy_run/haproxy.sock http://localhost/ -o /dev/null -v

open http://127.0.0.1:6003/
```
