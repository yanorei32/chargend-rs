# discardd-rs (discard daemon rs)

It's a implementation of "TCP Based Discard Service" [Discard Protocol [RFC863]](https://datatracker.ietf.org/doc/html/rfc863).

## Examples

### Server

```
./discardd-rs # default port usage (0.0.0.0:9)
./discardd-rs 0.0.0.0:9999 # specific port usage
```

NOTE: 9 is a well-known port. It may require administrative permission.
