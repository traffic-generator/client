# Network Traffic Generator Client

*A simple network traffic generator written in rust*

## Architecture

> **This is a work in progress**

The client exists out of 3 parts.

1. A **deamon** listens for commands from a command and control server.
1. A **generator** crafts packet data based on a desired protocol.
1. A **sender** transmits this data over the internet.

Note that there are multiple senders for different protocols, these have **nothing** to do with the generator and only try to **legitimately** send it out using the senders's protocol.
