[![progress-banner](https://backend.codecrafters.io/progress/redis/bbb1d029-ae01-4a52-8a51-6d18cdefdffc)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

This is a starting point for Rust solutions to the
["Build Your Own Redis" Challenge](https://codecrafters.io/challenges/redis).

In this challenge, you'll build a toy Redis clone that's capable of handling
basic commands like `PING`, `SET` and `GET`. Along the way we'll learn about
event loops, the Redis protocol and more.

**Note**: If you're viewing this repo on GitHub, head over to
[codecrafters.io](https://codecrafters.io) to try the challenge.

- [Introduction](#introduction)
- [Repository Setup](#repository-setup)
- [Passing the first stage](#passing-the-first-stage)
- [Stage 2 \& beyond](#stage-2--beyond)
- [Functionalities implemented for each stage](#functionalities-implemented-for-each-stage)
  - [Basic functionality](#basic-functionality)
    - [Stage 1: Bind to a port](#stage-1-bind-to-a-port)
      - [Your Task](#your-task)
      - [Tests](#tests)
      - [Notes](#notes)
    - [Stage 2: Respond to PING](#stage-2-respond-to-ping)
      - [Prerequisites](#prerequisites)
      - [Your Task](#your-task-1)
      - [Tests](#tests-1)
      - [Notes](#notes-1)
    - [Stage 3: Respond to multiple PINGs](#stage-3-respond-to-multiple-pings)
      - [Your Task](#your-task-2)
      - [Tests](#tests-2)
      - [Notes](#notes-2)
    - [Stage 4: Handle concurrent clients](#stage-4-handle-concurrent-clients)
      - [Your Task](#your-task-3)
      - [Tests](#tests-3)
      - [Notes](#notes-3)
    - [Stage 5: Implement the ECHO command](#stage-5-implement-the-echo-command)
      - [Your Task](#your-task-4)
      - [Tests](#tests-4)
      - [Notes](#notes-4)
    - [Stage 6: Implement the SET \& GET commands](#stage-6-implement-the-set--get-commands)
      - [Your Task](#your-task-5)
      - [Tests](#tests-5)
      - [Notes](#notes-5)
    - [Stage 7: Expiry](#stage-7-expiry)
      - [Your Task](#your-task-6)
      - [Tests](#tests-6)
      - [Notes](#notes-6)
  - [Replication](#replication)
    - [Stage 8: Configure listening port](#stage-8-configure-listening-port)
      - [Your Task](#your-task-7)
      - [Tests](#tests-7)
      - [Notes](#notes-7)
    - [Stage 9: The INFO command](#stage-9-the-info-command)
      - [Your Task](#your-task-8)
      - [The replication section](#the-replication-section)
      - [Tests](#tests-8)
      - [Notes](#notes-8)
    - [Stage 10: The INFO command on a replica](#stage-10-the-info-command-on-a-replica)
      - [Your Task](#your-task-9)
      - [The `--replicaof` flag](#the---replicaof-flag)
      - [Tests](#tests-9)
      - [Notes](#notes-9)

# Introduction

Welcome to the Build your own Redis challenge!

Redis is an in-memory data structure store often used as a database, cache, message broker and streaming engine. In this challenge you'll build your own Redis server that is capable of serving basic commands, reading RDB files and more.

Along the way, you'll learn about TCP servers, the Redis Protocol and more.

# Repository Setup

We've prepared a starter repository with some Rust code for you.

Step 1: Clone the repository.

```sh
git clone https://git.codecrafters.io/a6d6aeeafdfc7c25 codecrafters-redis-rust && cd codecrafters-redis-rust
```

Step 2: Push an empty commit.

```sh
git commit --allow-empty -m 'test' && git push origin master
```

When you run the above command, the "Listening for a git push" message below will change, and the first stage will be activated.

# Passing the first stage

The entry point for your Redis implementation is in `src/main.rs`. Study and
uncomment the relevant code, and push your changes to pass the first stage:

```sh
git add .
git commit -m "pass 1st stage" # any msg
git push origin master
```

That's all!

# Stage 2 & beyond

Note: This section is for stages 2 and beyond.

1. Ensure you have `cargo (1.54)` installed locally
1. Run `./spawn_redis_server.sh` to run your Redis server, which is implemented
   in `src/main.rs`. This command compiles your Rust project, so it might be
   slow the first time you run it. Subsequent runs will be fast.
1. Commit your changes and run `git push origin master` to submit your solution
   to CodeCrafters. Test output will be streamed to your terminal.

# Functionalities implemented for each stage

## Basic functionality

Here are the functionalities that you'll need to implement for each stage:

### Stage 1: Bind to a port

#### Your Task

In this stage, you'll implement a TCP server that listens on port 6379.

[TCP](https://en.wikipedia.org/wiki/Transmission_Control_Protocol) is the underlying protocol used by protocols like HTTP, SSH and others you're probably familiar with. Redis clients & servers use TCP to communicate with each other.

Don't worry if you're unfamiliar with the TCP protocol, or what Redis clients & servers are. You'll learn more about this in the next stages.

#### Tests

The tester will execute your program like this:

```sh
$ ./spawn_redis_server.sh
```

It'll then try to connect to your TCP server on port 6379. If the connection succeeds, you'll pass this stage.

#### Notes

- 6379 is the default port that Redis uses.
- If you already have a Redis server running on your machine and listening on port 6379, you'll see a "port already in use" error when running your code. Try stopping the existing Redis server and running your code again.

### Stage 2: Respond to PING

#### Prerequisites

Before attempting this stage, we recommend familiarizing yourself with:

- The TCP protocol
- Rust's `std::net` module
- How to write TCP servers in Rust

Our interactive concepts can help with this:

- [TCP: An Overview](https://app.codecrafters.io/concepts/tcp-overview) — Learn about the TCP protocol and how it works
- [TCP Servers in Rust](https://app.codecrafters.io/concepts/rust-tcp-server) — Learn how to write TCP servers using Rust's std::net module

#### Your Task

In this stage, you'll implement support for the [PING](https://redis.io/commands/ping) command.

Redis clients communicate with Redis servers by sending "[commands](https://redis.io/commands/)". For each command, a Redis server sends a response back to the client. Commands and responses are both encoded using the [Redis protocol](https://redis.io/topics/protocol) (we'll learn more about this in later stages).

[PING](https://redis.io/commands/ping/) is one of the simplest Redis commands. It's used to check whether a Redis server is healthy.

The response for the `PING` command is `+PONG\r\n`. This is the string "PONG" encoded using the [Redis protocol](https://redis.io/docs/reference/protocol-spec/).

In this stage, we'll cut corners by ignoring client input and hardcoding `+PONG\r\n` as a response. We'll learn to parse client input in later stages.

#### Tests

The tester will execute your program like this:

```sh
$ ./spawn_redis_server.sh
```

It'll then send a `PING` command to your server and expect a `+PONG\r\n` response.

```sh
$ redis-cli ping
```

Your server should respond with `+PONG\r\n`, which is "PONG" encoded as a [RESP simple string](https://redis.io/docs/reference/protocol-spec/#resp-simple-strings).

#### Notes

- You can ignore the data that the tester sends you for this stage. We'll get to parsing client input in later stages. For now, you can just hardcode `+PONG\r\n` as the response.
- You can also ignore handling multiple clients and handling multiple PING commands in the stage, we'll get to that in later stages.
- The exact bytes your program will receive won't be just `ping`, you'll receive something like this: `\*1\r\n$4\r\nping\r\n`, which is the Redis protocol encoding of the `PING` command. We'll learn more about this in later stages.

### Stage 3: Respond to multiple PINGs

#### Your Task

In this stage, you'll respond to multiple [PING](https://redis.io/commands/ping) commands sent by the same connection.

A Redis server starts to listen for the next command as soon as it's done responding to the previous one. This allows Redis clients to send multiple commands using the same connection.

#### Tests

The tester will execute your program like this:

```bash
$ ./spawn_redis_server.sh
```

It'll then send two PING commands using the same connection:

```bash
$ echo -e "ping\nping" | redis-cli
```

The tester will expect to receive two `+PONG\r\n` responses.

You'll need to run a loop that reads input from a connection and sends a response back.

#### Notes

- Just like the previous stage, you can hardcode `+PONG\r\n` as the response for this stage. We'll get to parsing client input in later stages.
- The two PING commands will be sent using the same connection. We'll get to handling multiple connections in later stages.

### Stage 4: Handle concurrent clients

#### Your Task

In this stage, you'll add support for multiple concurrent clients.

In addition to handling multiple commands from the same client, Redis servers are also designed to handle multiple clients at once.

To implement this, you'll need to either use threads, or, if you're feeling adventurous, an [Event Loop](https://en.wikipedia.org/wiki/Event_loop) (like the official Redis implementation does).

#### Tests

The tester will execute your program like this:

```bash
$ ./spawn_redis_server.sh
```

It'll then send two PING commands concurrently using two different connections:

```bash
## These two will be sent concurrently so that we test your server's ability to handle concurrent clients.
$ redis-cli ping
$ redis-cli ping
```

The tester will expect to receive two `+PONG\r\n` responses.

#### Notes

- Since the tester client _only_ sends the PING command at the moment, it's okay to ignore what the client sends and hardcode a response. We'll get to parsing client input in later stages.

### Stage 5: Implement the ECHO command

#### Your Task

In this stage, you'll add support for the [ECHO](https://redis.io/commands/echo) command.

`ECHO` is a command like `PING` that's used for testing and debugging. It accepts a single argument and returns it back as a RESP bulk string.

```bash
$ redis-cli ping ## The command you implemented in previous stages
PONG
$ redis-cli echo hey ## The command you'll implement in this stage
hey
```

#### Tests

The tester will execute your program like this:

```bash
$ ./spawn_redis_server.sh
```

It'll then send an `ECHO` command with an argument to your server:

```bash
$ redis-cli echo hey
```

The tester will expect to receive `$3\r\nhey\r\n` as a response (that's the string `hey` encoded as a [RESP bulk string](https://redis.io/docs/reference/protocol-spec/#bulk-strings).

#### Notes

- We suggest that you implement a proper Redis protocol parser in this stage. It'll come in handy in later stages.
- Redis command names are case-insensitive, so `ECHO`, `echo` and `EcHo` are all valid commands.
- The tester will send a random string as an argument to the `ECHO` command, so you won't be able to hardcode the response to pass this stage.
- The exact bytes your program will receive won't be just `echo hey`, you'll receive something like this: `*2\r\n$4\r\necho\r\n$3\r\nhey\r\n`. That's `["echo", "hey"]` encoded using the [Redis protocol](https://redis.io/docs/reference/protocol-spec/).
- You can read more about how "commands" are handled in the Redis protocol [here](https://redis.io/docs/reference/protocol-spec/#sending-commands-to-a-redis-server).

### Stage 6: Implement the SET & GET commands

#### Your Task

In this stage, you'll add support for the [SET](https://redis.io/commands/set) & [GET](https://redis.io/commands/get) commands.

The `SET` command is used to set a key to a value. The `GET` command is used to retrieve the value of a key.

```bash
$ redis-cli set foo bar
OK
$ redis-cli get foo
bar
```

The `SET` command supports a number of extra options like `EX` (expiry time in seconds), `PX` (expiry time in milliseconds) and more. We won't cover these extra options in this stage. We'll get to them in later stages.

#### Tests

The tester will execute your program like this:

```bash
./spawn_redis_server.sh
```

It'll then send a `SET` command to your server:

```bash
$ redis-cli set foo bar
```

The tester will expect to receive `+OK\r\n` as a response (that's the string `OK` encoded as a [RESP simple string](https://redis.io/docs/reference/protocol-spec/#resp-simple-strings)).

This command will be followed by a `GET` command:

```bash
$ redis-cli get foo
```

The tester will expect to receive `$3\r\nbar\r\n` as a response (that's the string `bar` encoded as a [RESP bulk string](https://redis.io/docs/reference/protocol-spec/#bulk-strings).

#### Notes

- If you implemented a proper Redis protocol parser in the previous stage, you should be able to reuse it in this stage.
- Just like the previous stage, the values used for keys and values will be random, so you won't be able to hardcode the response to pass this stage.
- If a key doesn't exist, the `GET` command should return a "null bulk string" (`$-1\r\n`). We won't explicitly test this in this stage, but you'll need it for the next stage (expiry).

### Stage 7: Expiry

#### Your Task

In this stage, you'll add support for setting a key with an expiry.

The expiry for a key can be provided using the "PX" argument to the [SET](https://redis.io/commands/set) command. The expiry is provided in milliseconds.

```bash
$ redis-cli set foo bar px 100 ## Sets the key "foo" to "bar" with an expiry of 100 milliseconds
OK
```

After the key has expired, a `GET` command for that key should return a "null bulk string" (`$-1\r\n`).

#### Tests

The tester will execute your program like this:

```bash
$ ./spawn_redis_server.sh
```

It'll then send a `SET` command to your server to set a key with an expiry:

```bash
$ redis-cli set foo bar px 100
```

It'll then immediately send a `GET` command to retrieve the value:

```bash
$ redis-cli get foo
```

It'll expect the response to be `bar` (encoded as a RESP bulk string).

It'll then wait for the key to expire and send another `GET` command:

```bash
$ sleep 0.2 && redis-cli get foo
```

It'll expect the response to be `$-1\r\n` (a "null bulk string").

#### Notes

- Just like command names, command arguments are also case-insensitive. So `PX`, `px` and `pX` are all valid.
- The keys, values and expiry times used in the tests will be random, so you won't be able to hardcode a response to pass this stage.

## Replication

### Stage 8: Configure listening port

#### Your Task

Welcome to the Replication extension!

In this extension, you'll extend your Redis server to support [leader-follower replication](https://redis.io/docs/management/replication/). You'll be able to run multiple Redis servers with one acting as the "master" and the others as "replicas". Changes made to the master will be automatically replicated to replicas.

Since we'll need to run multiple instances of your Redis server at once, we can't run all of them on port 6379.

In this stage, you'll add support for starting the Redis server on a custom port. The port number will be passed to your program via the `--port` flag.

#### Tests

The tester will execute your program like this:

```bash
./spawn_redis_server.sh --port 6380
```

It'll then try to connect to your TCP server on the specified port number (`6380` in the example above). If the connection succeeds, you'll pass this stage.

#### Notes

- Your program still needs to pass the previous stages, so if `--port` isn't specified, you should default to port 6379.
- The tester will pass a random port number to your program, so you can't hardcode the port number from the example above.
- If your repository was created before 5th Oct 2023, it's possible that your `./spawn_redis_server.sh` script might not be passing arguments on to your program. You'll need to edit `./spawn_redis_server.sh` to fix this, check [this PR](https://github.com/codecrafters-io/build-your-own-redis/pull/89/files) for details.

### Stage 9: The INFO command

#### Your Task

In this stage, you'll add support for the [INFO](https://redis.io/commands/info/) command.

The `INFO` command returns information and statistics about a Redis server. In this stage, we'll add support for the `replication` section of the `INFO` command.

#### The replication section

When you run the `INFO` command against a Redis server, you'll see something like this:

```bash
$ redis-cli info replication
# Replication
role:master
connected_slaves:0
master_replid:8371b4fb1155b71f4a04d3e1bc3e18c4a990aeeb
master_repl_offset:0
second_repl_offset:-1
repl_backlog_active:0
repl_backlog_size:1048576
repl_backlog_first_byte_offset:0
repl_backlog_histlen:
```

The reply to this command is a [Bulk string](https://redis.io/docs/reference/protocol-spec/#bulk-strings) where each line is a key value pair, separated by ":".

Here are what some of the important fields mean:

- `role`: The role of the server (`master` or `slave`)
- `connected_slaves`: The number of connected replicas
- `master_replid`: The replication ID of the master (we'll get to this in later stages)
- `master_repl_offset`: The replication offset of the master (we'll get to this in later stages)

In this stage, you'll only need to support the `role` key. We'll add support for other keys in later stages.

#### Tests

The tester will execute your program like this:

```bash
./spawn_redis_server.sh --port <PORT>
```

It'll then send the `INFO` command with `replication` as an argument.

```bash
$ redis-cli -p <PORT> info replication
```

Your program should respond with a [Bulk string](https://redis.io/docs/reference/protocol-spec/#bulk-strings) where each line is a key value pair separated by `:`. The tester will only look for the `role` key, and assert that the value is `master`.

#### Notes

- In the response for the `INFO` command, you only need to support the `role` key for this stage. We'll add support for the other keys in later stages.
- The `# Replication` heading in the response is optional, you can ignore it.
- The response to `INFO` needs to be encoded as a [Bulk string](https://redis.io/docs/reference/protocol-spec/#bulk-strings).
  - An example valid response would be `$11\r\nrole:master\r\n` (the string `role:master` encoded as a [Bulk string](https://redis.io/docs/reference/protocol-spec/#bulk-strings))
- The `INFO` command can be used without any arguments, in which case it returns all sections available. In this stage, we'll always send `replication` as an argument to the `INFO` command, so you only need to support the `replication` section.

### Stage 10: The INFO command on a replica

#### Your Task

In this stage, you'll extend your [INFO](https://redis.io/commands/info/) command to run on a replica.

#### The `--replicaof` flag

By default, a Redis server assumes the "master" role. When the `--replicaof` flag is passed, the server assumes the "slave" role instead.

Here's an example usage of the `--replicaof` flag:

```bash
./spawn_redis_server.sh --port 6380 --replicaof localhost 6379
```

In this example, we're starting a Redis server in replica mode. The server itself will listen for connections on port 6380, but it'll also connect to a master (another Redis server) running on localhost port 6379 and replicate all changes from the master.

We'll learn more about how this replication works in later stages. For now, we'll focus on adding support for the `--replicaof` flag, and extending the `INFO` command to support returning `role: slave` when the server is a replica.

#### Tests

The tester will execute your program like this:

```bash
./spawn_redis_server.sh --port <PORT> --replicaof <MASTER_HOST> <MASTER_PORT>
```

It'll then send the `INFO` command with `replication` as an argument to your server.

```bash
$ redis-cli -p <PORT> info replication
```

Your program should respond with a [Bulk string](https://redis.io/docs/reference/protocol-spec/#bulk-strings) where each line is a key value pair separated by `:`. The tester will only look for the `role` key, and assert that the value is `slave`.

#### Notes

- Your program still needs to pass the previous stage tests, so if `--replicaof` isn't specified, you should default to the `master` role.
- Just like the last stage, you only need to support the `role` key in the response for this stage. We'll add support for the other keys in later stages.
- You don't need to actually connect to the master server specified via `--replicaof` in this stage. We'll get to that in later stages.
