# Telnet Chatroom

**Telnet Chatroom** is a sample application in Rust to develop a chat server based on Telnet. Participants can connect to the server, pick a username, exchange messages with all other users.

Usage:

- `list`: lists the participants in the room
- `anything else`: sends the typed text as a message to all other participants.

Example:

Note: commands typed by the user are in **bold**; server output is in _italic_.

Server side:

<pre>
<code>
<b>cargo run</b>
</code>
</pre>

Client 1:

<pre>
<code><b>telnet 127.0.0.1 8080</b>
<i>Trying 127.0.0.1...
Connected to 127.0.0.1.
Escape character is '^]'.
What is your name?</i>
<b>Gandalf</b>
<i>Welcome to the chat room, Gandalf! There is no else here. You can send new messages anytime.
Legolas (1): Hello Gandalf</i></code>
</pre>

Client 2:

<pre>
<code><b>telnet 127.0.0.1 8080</b>
<i>Trying 127.0.0.1...
Connected to 127.0.0.1.
Escape character is '^]'.
What is your name?</i>
<b>Legolas</b>
<i>Welcome to the chat room, Legolas! There are 1 other people here:
        Gandalf (0)</i>
<b>Hello Gandalf
list</b>
<i>Participants in the room:
        Gandalf (0)</i></code>
</pre>
