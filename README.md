[![progress-banner](https://backend.codecrafters.io/progress/redis/bbb1d029-ae01-4a52-8a51-6d18cdefdffc)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

This is a starting point for Rust solutions to the
["Build Your Own Redis" Challenge](https://codecrafters.io/challenges/redis).

In this challenge, you'll build a toy Redis clone that's capable of handling
basic commands like `PING`, `SET` and `GET`. Along the way we'll learn about
event loops, the Redis protocol and more.

**Note**: If you're viewing this repo on GitHub, head over to
[codecrafters.io](https://codecrafters.io) to try the challenge.

# Introduction

Welcome to the Build your own Redis challenge!

Redis is an in-memory data structure store often used as a database, cache, message broker and streaming engine. In this challenge you'll build your own Redis server that is capable of serving basic commands, reading RDB files and more.

Along the way, you'll learn about TCP servers, the Redis Protocol and more.

# Repository Setup

We've prepared a starter repository with some Rust code for you.

Step 1: Clone the repository.

````sh
git clone https://git.codecrafters.io/a6d6aeeafdfc7c25 codecrafters-redis-rust && cd codecrafters-redis-rust```

Step 2: Push an empty commit.

```sh
git commit --allow-empty -m 'test' && git push origin master```

When you run the above command, the "Listening for a git push" message below will change, and the first stage will be activated.

# Passing the first stage

The entry point for your Redis implementation is in `src/main.rs`. Study and
uncomment the relevant code, and push your changes to pass the first stage:

```sh
git add .
git commit -m "pass 1st stage" # any msg
git push origin master```

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

Here are the functionalities that you'll need to implement for each stage:

## Stage 1: Bind to a port

### Your Task

In this stage, you'll implement a TCP server that listens on port 6379.

[TCP](https://en.wikipedia.org/wiki/Transmission_Control_Protocol) is the underlying protocol used by protocols like HTTP, SSH and others you're probably familiar with. Redis clients & servers use TCP to communicate with each other.

Don't worry if you're unfamiliar with the TCP protocol, or what Redis clients & servers are. You'll learn more about this in the next stages.

#### Tests

The tester will execute your program like this:

```sh
$ ./spawn_redis_server.sh```

It'll then try to connect to your TCP server on port 6379. If the connection succeeds, you'll pass this stage.

#### Notes

- 6379 is the default port that Redis uses.
- If you already have a Redis server running on your machine and listening on port 6379, you'll see a "port already in use" error when running your code. Try stopping the existing Redis server and running your code again.

## Stage 2: Respond to PING

### Prerequisites

Before attempting this stage, we recommend familiarizing yourself with:

- The TCP protocol
- Rust's `std::net` module
- How to write TCP servers in Rust

Our interactive concepts can help with this:

- [TCP: An Overview](https://app.codecrafters.io/concepts/tcp-overview) — Learn about the TCP protocol and how it works
- [TCP Servers in Rust](https://app.codecrafters.io/concepts/rust-tcp-server) — Learn how to write TCP servers using Rust's std::net module

### Your Task

In this stage, you'll implement support for the [PING](https://redis.io/commands/ping) command.

Redis clients communicate with Redis servers by sending "[commands](https://redis.io/commands/)". For each command, a Redis server sends a response back to the client. Commands and responses are both encoded using the [Redis protocol](https://redis.io/topics/protocol) (we'll learn more about this in later stages).

[PING](https://redis.io/commands/ping/) is one of the simplest Redis commands. It's used to check whether a Redis server is healthy.

The response for the `PING` command is `+PONG\r\n`. This is the string "PONG" encoded using the [Redis protocol](https://redis.io/docs/reference/protocol-spec/).

In this stage, we'll cut corners by ignoring client input and hardcoding `+PONG\r\n` as a response. We'll learn to parse client input in later stages.

#### Tests

The tester will execute your program like this:

```sh
$ ./spawn_redis_server.sh```

It'll then send a `PING` command to your server and expect a `+PONG\r\n` response.

```sh
$ redis-cli ping```

Your server should respond with `+PONG\r\n`, which is "PONG" encoded as a [RESP simple string](https://redis.io/docs/reference/protocol-spec/#resp-simple-strings).

#### Notes

- You can ignore the data that the tester sends you for this stage. We'll get to parsing client input in later stages. For now, you can just hardcode `+PONG\r\n` as the response.
- You can also ignore handling multiple clients and handling multiple PING commands in the stage, we'll get to that in later stages.
- The exact bytes your program will receive won't be just `ping`, you'll receive something like this: `\*1\r\n$4\r\nping\r\n`, which is the Redis protocol encoding of the `PING` command. We'll learn more about this in later stages.

## Stage 3: Respond to multiple PINGs

### Your Task

In this stage, you'll respond to multiple [PING](https://redis.io/commands/ping) commands sent by the same connection.

A Redis server starts to listen for the next command as soon as it's done responding to the previous one. This allows Redis clients to send multiple commands using the same connection.

### Tests

The tester will execute your program like this:

```bash
$ ./spawn_redis_server.sh
````

It'll then send two PING commands using the same connection:

```bash
$ echo -e "ping\nping" | redis-cli
```

The tester will expect to receive two `+PONG\r\n` responses.

You'll need to run a loop that reads input from a connection and sends a response back.

### Notes

- Just like the previous stage, you can hardcode `+PONG\r\n` as the response for this stage. We'll get to parsing client input in later stages.
- The two PING commands will be sent using the same connection. We'll get to handling multiple connections in later stages.

## Stage 4: Handle concurrent clients

### Your Task

In this stage, you'll add support for multiple concurrent clients.

In addition to handling multiple commands from the same client, Redis servers are also designed to handle multiple clients at once.

To implement this, you'll need to either use threads, or, if you're feeling adventurous, an [Event Loop](https://en.wikipedia.org/wiki/Event_loop) (like the official Redis implementation does).

### Tests

The tester will execute your program like this:

```bash
$ ./spawn_redis_server.sh
```

It'll then send two PING commands concurrently using two different connections:

```bash
# These two will be sent concurrently so that we test your server's ability to handle concurrent clients.
$ redis-cli ping
$ redis-cli ping
```

The tester will expect to receive two `+PONG\r\n` responses.

### Notes

- Since the tester client _only_ sends the PING command at the moment, it's okay to ignore what the client sends and hardcode a response. We'll get to parsing client input in later stages.

## Stage 5: Implement the ECHO command

### Your Task

In this stage, you'll add support for the [ECHO](https://redis.io/commands/echo) command.

`ECHO` is a command like `PING` that's used for testing and debugging. It accepts a single argument and returns it back as a RESP bulk string.

```bash
$ redis-cli ping # The command you implemented in previous stages
PONG
$ redis-cli echo hey # The command you'll implement in this stage
hey
```

### Tests

The tester will execute your program like this:

```bash
$ ./spawn_redis_server.sh
```

It'll then send an `ECHO` command with an argument to your server:

```bash
$ redis-cli echo hey
```

The tester will expect to receive `$3\r\nhey\r\n` as a response (that's the string `hey` encoded as a [RESP bulk string](https://redis.io/docs/reference/protocol-spec/#bulk-strings).

### Notes

- We suggest that you implement a proper Redis protocol parser in this stage. It'll come in handy in later stages.
- Redis command names are case-insensitive, so `ECHO`, `echo` and `EcHo` are all valid commands.
- The tester will send a random string as an argument to the `ECHO` command, so you won't be able to hardcode the response to pass this stage.
- The exact bytes your program will receive won't be just `echo hey`, you'll receive something like this: `*2\r\n$4\r\necho\r\n$3\r\nhey\r\n`. That's `["echo", "hey"]` encoded using the [Redis protocol](https://redis.io/docs/reference/protocol-spec/).
- You can read more about how "commands" are handled in the Redis protocol [here](https://redis.io/docs/reference/protocol-spec/#sending-commands-to-a-redis-server).

## Stage 6: Implement the SET & GET commands

## Your Task

In this stage, you'll add support for the [SET](https://redis.io/commands/set) & [GET](https://redis.io/commands/get) commands.

The `SET` command is used to set a key to a value. The `GET` command is used to retrieve the value of a key.

```bash
$ redis-cli set foo bar
OK
$ redis-cli get foo
bar
```

The `SET` command supports a number of extra options like `EX` (expiry time in seconds), `PX` (expiry time in milliseconds) and more. We won't cover these extra options in this stage. We'll get to them in later stages.

### Tests

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

### Notes

- If you implemented a proper Redis protocol parser in the previous stage, you should be able to reuse it in this stage.
- Just like the previous stage, the values used for keys and values will be random, so you won't be able to hardcode the response to pass this stage.
- If a key doesn't exist, the `GET` command should return a "null bulk string" (`$-1\r\n`). We won't explicitly test this in this stage, but you'll need it for the next stage (expiry).
