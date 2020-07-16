# Tortue

A rusty torrent client built for speed and security.

## Components

### Bencode

Parser and (de)serializer implementation of the bencode encoding scheme. It seems fully featured and provides parsing at 3.2GiB/s (on my machine) and deserialization at 1.6 GiB/s. Writing to bytes is done at 3.75 GiB/s and serializing is done at 4.8 GiB/s, combined (serialization to bytes) at 1.93 GiB/s

### Protocol

Async implementation of the torrent protocol

