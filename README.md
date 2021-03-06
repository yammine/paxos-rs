# Paxos-rs 

[![Travis](https://travis-ci.org/zowens/paxos-rs.svg?branch=master)](https://travis-ci.org/zowens/paxos-rs/)

Library encapsulating Multi-Decree Paxos and variants in Rust. The core algorithm is embeddable in other systems that need a replicated state machine.

*Development Status*: Alpha (not for production use yet)

The library is getting close to a [0.1 release](https://github.com/zowens/paxos-rs/milestone/1).

## Example Key-Value Store

To try out the algorithm, an [example](examples/http-paxos) implements an HTTP transport for both clients and the replicas.

### Running
```bash
$ cargo build --example http-paxos
$ ./target/debug/examples/http-paxos 0 &
$ ./target/debug/examples/http-paxos 1 &
$ ./target/debug/examples/http-paxos 2 &
```

### Example
```bash
# GET the "foo" key which does not exist yet
$ curl localhost:8080/foo -i
HTTP/1.1 404 Not Found
content-length: 0
date: Tue, 09 Jun 2020 19:54:19 GMT

# SET the key to a value by passing the value in the body
$ curl localhost:8080/foo -d 'hello paxos' -i
HTTP/1.1 204 No Content
x-paxos-slot: 3
content-length: 0
date: Tue, 09 Jun 2020 19:55:29 GMT

# Now run a GET again on the key to get the value
$ curl localhost:8080/foo -i
HTTP/1.1 200 OK
x-paxos-slot: 4
content-length: 11
date: Tue, 09 Jun 2020 19:56:41 GMT

hello paxos
```

## References
* [Paxos Variants](http://paxos.systems/variants.html#mencius)
* [Understanding Paxos](https://understandingpaxos.wordpress.com/)
* [Paxos Made Moderately Complex](http://paxos.systems/)
* [Flexible Quorums](https://fpaxos.github.io/)
* [WPaxos](https://muratbuffalo.blogspot.com/2017/12/wpaxos-wide-area-network-paxos-protocol.html)
* [PigPaxos](https://arxiv.org/pdf/2003.07760.pdf)
