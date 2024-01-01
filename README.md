# WhackSockets

Simple recreation of the WebSocket Protocol in rust.

## Reasoning
I work a lot with websockets, majority of the time to connect to live data, sometimes in rust, most often if not Typescript.
However, using libraries like tungstenite and ws almost always abstract all the gritty details away from the developer.<br>
I am trying to get my hands dirty with the WebSocket Protocol.

## What I've learnt

1. Bit manipulation
    - Really cool, using the bitAND operator `&` with bits that are ones lets you essentially sum the binary.
    e.g:
    ```rust
    let x: u8 = 0b0000_0010;
    let y: u8 = 0b1111_1111;

    println!("{}", x & y); // This prints 2 (effectively summing up the binary x);

    ```
    There really isn't much to say here it is pretty surface level bit manipulation but this is the first time I'm working with this so it's pretty cool.
    
2. Data Frames
    - WebSockets communicate with Data Frames, I like to think of them as essentially one of those puzzle boxes that once you've solved you get access to the content.

3. TCP
    - Nothing too fancy, just learnt more about the TCP protocol, maybe one day I'll rebuilt TCP from scratch as a side project?
