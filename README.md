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

Here are the functionalities that you'll need to implement for each stage:

## Stage 1: Bind to a port

### Your Task

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

## Stage 3: Respond to multiple PINGs

In this stage, you'll respond to multiple [PING](https://redis.io/commands/ping) commands sent by the same connection.

A Redis server starts to listen for the next command as soon as it's done responding to the previous one. This allows Redis clients to send multiple commands using the same connection.

### Tests

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

### Notes

- Just like the previous stage, you can hardcode `+PONG\r\n` as the response for this stage. We'll get to parsing client input in later stages.
- The two PING commands will be sent using the same connection. We'll get to handling multiple connections in later stages.
